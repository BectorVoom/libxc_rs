//! LDA_K_ZLP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
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
pub fn lda_k_zlp_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t2 * t5 * t7;
            let t9 = v_rho0 - v_rho1;
            let t10 = v_rho0 + v_rho1;
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t13 = f64x8::splat(1.0) + t12;
            let t14 = (t13).simd_le(zeta_threshold);
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = ((t14).select(t17, t19 * t13));
            let t22 = f64x8::splat(1.0) - t12;
            let t23 = (t22).simd_le(zeta_threshold);
            let t24 = (simd::cbrt(t22));
            let t25 = t24 * t24;
            let t27 = ((t23).select(t17, t25 * t22));
            let t29 = t21 / f64x8::splat(2.0) + t27 / f64x8::splat(2.0);
            let t30 = (simd::cbrt(t10));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(1.0) / t30;
            let t35 = f64x8::splat(1.0) + f64x8::splat(510.2040816326531) * t33;
            let t36 = (simd::ln(t35));
            let t39 = f64x8::splat(1.0) - f64x8::splat(0.00196) * t30 * t36;
            let t41 = t8 * t32 * t39;
            let tzk0 = f64x8::splat(1.0790666666666666) * t41;
            acc_zk = tzk0;
            let t42 = f64x8::splat(1.7984444444444445) * t41;
            let t43 = t31 * t10;
            let t45 = t43 * t2 * t5;
            let t46 = t10 * t10;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t9 * t47;
            let t49 = t11 - t48;
            let t52 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t49));
            let t53 = -t49;
            let t56 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t53));
            let t58 = t52 / f64x8::splat(2.0) + t56 / f64x8::splat(2.0);
            let t59 = t7 * t58;
            let t63 = t7 * t29;
            let t67 = f64x8::splat(1.0) / t35;
            let t70 = -f64x8::splat(0.0006533333333333333) / t31 * t36 + f64x8::splat(0.3333333333333333) * t11 * t67;
            let t73 = f64x8::splat(1.0790666666666666) * t45 * t63 * t70;
            let tvrho0 = t42 + f64x8::splat(1.0790666666666666) * t45 * t59 * t39 + t73;
            acc_vrho_0 = tvrho0;
            let t74 = -t11 - t48;
            let t77 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t74));
            let t78 = -t74;
            let t81 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t78));
            let t84 = t7 * (t77 / f64x8::splat(2.0) + t81 / f64x8::splat(2.0));
            let t85 = t84 * t39;
            let tvrho1 = t42 + f64x8::splat(1.0790666666666666) * t45 * t85 + t73;
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}
