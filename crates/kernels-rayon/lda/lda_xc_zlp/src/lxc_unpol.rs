//! LDA_XC_ZLP lxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_zlp_lxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        {
            let t1 = (simd::cbrt(v_rho));
            let t4 = f64x8::splat(1.0) + f64x8::splat(105.5562709925034) / t1;
            let t5 = (simd::ln(t4));
            let t8 = f64x8::splat(1.0) - f64x8::splat(0.00947362) * t5 * t1;
            let t9 = t8 * t1;
            let tzk0 = -f64x8::splat(0.93222) * t9;
            acc_zk = tzk0;
            let t12 = t1 * v_rho;
            let t14 = f64x8::splat(1.0) / t4;
            let t17 = t1 * t1;
            let t18 = f64x8::splat(1.0) / t17;
            let t21 = f64x8::splat(0.3333333333333333) / v_rho * t14 - f64x8::splat(0.0031578733333333334) * t5 * t18;
            let tvrho0 = -f64x8::splat(1.24296) * t9 - f64x8::splat(0.93222) * t12 * t21;
            acc_vrho = tvrho0;
            let t28 = v_rho * v_rho;
            let t34 = t4 * t4;
            let t35 = f64x8::splat(1.0) / t34;
            let t39 = f64x8::splat(1.0) / t17 / v_rho;
            let t42 = -f64x8::splat(0.2222222222222222) / t28 * t14 + f64x8::splat(11.728474554722599) / t1 / t28 * t35 + f64x8::splat(0.002105248888888889) * t5 * t39;
            let tv2rho20 = -f64x8::splat(2.48592) * t21 * t1 - f64x8::splat(0.41432) * t8 * t18 - f64x8::splat(0.93222) * t12 * t42;
            acc_v2rho2 = tv2rho20;
            let t51 = t28 * v_rho;
            let t60 = f64x8::splat(1.0) / t17 / t51;
            let t62 = f64x8::splat(1.0) / t34 / t4;
            let t66 = f64x8::splat(1.0) / t17 / t28;
            let t69 = f64x8::splat(0.37037037037037035) / t51 * t14 - f64x8::splat(35.1854236641678) / t1 / t51 * t35 + f64x8::splat(825.3426922846528) * t60 * t62 - f64x8::splat(0.003508748148148148) * t5 * t66;
            let tv3rho30 = -f64x8::splat(3.72888) * t42 * t1 - f64x8::splat(1.24296) * t21 * t18 + f64x8::splat(0.2762133333333333) * t8 * t39 - f64x8::splat(0.93222) * t12 * t69;
            acc_v3rho3 = tv3rho30;
            let t80 = t28 * t28;
            let t94 = t34 * t34;
            let tv4rho40 = -f64x8::splat(4.97184) * t69 * t1 - f64x8::splat(2.48592) * t42 * t18 + f64x8::splat(1.1048533333333332) * t21 * t39 - f64x8::splat(0.4603555555555556) * t8 * t66 - f64x8::splat(0.93222) * t12 * (-f64x8::splat(0.9876543209876543) / t80 * t14 + f64x8::splat(130.3163839413622) / t1 / t80 * t35 - f64x8::splat(5502.2846152310185) / t17 / t80 * t62 + f64x8::splat(87120.09688848116) / t80 / v_rho / t94 + f64x8::splat(0.009356661728395062) * t5 * t60);
            acc_v4rho4 = tv4rho40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        ip += 8;
    }
}
