//! LDA_XC_ZLP lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_zlp_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = (simd::cbrt(t1));
            let t5 = f64x8::splat(1.0) + f64x8::splat(105.5562709925034) / t2;
            let t6 = (simd::ln(t5));
            let t9 = f64x8::splat(1.0) - f64x8::splat(0.00947362) * t6 * t2;
            let t10 = t9 * t2;
            let tzk0 = -f64x8::splat(0.93222) * t10;
            acc_zk = tzk0;
            let t13 = t2 * t1;
            let t15 = f64x8::splat(1.0) / t5;
            let t18 = t2 * t2;
            let t19 = f64x8::splat(1.0) / t18;
            let t22 = f64x8::splat(0.3333333333333333) / t1 * t15 - f64x8::splat(0.0031578733333333334) * t6 * t19;
            let tvrho0 = -f64x8::splat(1.24296) * t10 - f64x8::splat(0.93222) * t13 * t22;
            acc_vrho_0 = tvrho0;
            let tvrho1 = tvrho0;
            acc_vrho_1 = tvrho1;
            let t29 = t1 * t1;
            let t35 = t5 * t5;
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = f64x8::splat(1.0) / t18 / t1;
            let t43 = -f64x8::splat(0.2222222222222222) / t29 * t15 + f64x8::splat(11.728474554722599) / t2 / t29 * t36 + f64x8::splat(0.002105248888888889) * t6 * t40;
            let tv2rho20 = -f64x8::splat(2.48592) * t22 * t2 - f64x8::splat(0.41432) * t9 * t19 - f64x8::splat(0.93222) * t13 * t43;
            acc_v2rho2_0 = tv2rho20;
            let tv2rho21 = tv2rho20;
            acc_v2rho2_1 = tv2rho21;
            let tv2rho22 = tv2rho21;
            acc_v2rho2_2 = tv2rho22;
            let t52 = t29 * t1;
            let t61 = f64x8::splat(1.0) / t18 / t52;
            let t63 = f64x8::splat(1.0) / t35 / t5;
            let t67 = f64x8::splat(1.0) / t18 / t29;
            let t70 = f64x8::splat(0.37037037037037035) / t52 * t15 - f64x8::splat(35.1854236641678) / t2 / t52 * t36 + f64x8::splat(825.3426922846528) * t61 * t63 - f64x8::splat(0.003508748148148148) * t6 * t67;
            let tv3rho30 = -f64x8::splat(3.72888) * t43 * t2 - f64x8::splat(1.24296) * t22 * t19 + f64x8::splat(0.2762133333333333) * t9 * t40 - f64x8::splat(0.93222) * t13 * t70;
            acc_v3rho3_0 = tv3rho30;
            let tv3rho31 = tv3rho30;
            acc_v3rho3_1 = tv3rho31;
            let tv3rho32 = tv3rho31;
            acc_v3rho3_2 = tv3rho32;
            let tv3rho33 = tv3rho32;
            acc_v3rho3_3 = tv3rho33;
            let t81 = t29 * t29;
            let t95 = t35 * t35;
            let tv4rho40 = -f64x8::splat(4.97184) * t70 * t2 - f64x8::splat(2.48592) * t43 * t19 + f64x8::splat(1.1048533333333332) * t22 * t40 - f64x8::splat(0.4603555555555556) * t9 * t67 - f64x8::splat(0.93222) * t13 * (-f64x8::splat(0.9876543209876543) / t81 * t15 + f64x8::splat(130.3163839413622) / t2 / t81 * t36 - f64x8::splat(5502.2846152310185) / t18 / t81 * t63 + f64x8::splat(87120.09688848116) / t81 / t1 / t95 + f64x8::splat(0.009356661728395062) * t6 * t61);
            acc_v4rho4_0 = tv4rho40;
            let tv4rho41 = tv4rho40;
            acc_v4rho4_1 = tv4rho41;
            let tv4rho42 = tv4rho41;
            acc_v4rho4_2 = tv4rho42;
            let tv4rho43 = tv4rho42;
            acc_v4rho4_3 = tv4rho43;
            let tv4rho44 = tv4rho43;
            acc_v4rho4_4 = tv4rho44;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}
