//! MGGA_X_EDMGGA exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_edmgga.c`
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
pub fn mgga_x_edmgga_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t25 = (simd::cbrt(t24));
            let t27 = t21 * t22 * t25 / f64x8::splat(9.0);
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_tau * t30;
            let t32 = t19 * t19;
            let t34 = f64x8::splat(1.0) / t32 / v_rho;
            let t36 = v_sigma * t30;
            let t37 = v_rho * v_rho;
            let t39 = f64x8::splat(1.0) / t32 / t37;
            let t42 = v_lapl * t30;
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = t25 * t25;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = (t31 * t34 - t36 * t39 / f64x8::splat(8.0) - t42 * t34 / f64x8::splat(4.0)) * t46 * t49;
            let t51 = f64x8::splat(5.0) / f64x8::splat(9.0) * t50;
            let t52 = (-t51).simd_lt(-f64x8::splat(14205.545454545454));
            let t53 = f64x8::splat(0.39111111111111113) * t50;
            let t55 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7041420454545455) - t53);
            let t57 = ((t55).select(-f64x8::splat(0.00014204545454545454), f64x8::splat(0.704) - t53));
            let t60 = t57 * t57;
            let t61 = t60 * t57;
            let t62 = f64x8::splat(1.0) / t61;
            let t65 = f64x8::splat(1.0) - t51;
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.495616) * t66;
            let t69 = ((t68).sqrt());
            let t71 = ((t52).select(-f64x8::splat(1.0) / t57 / f64x8::splat(2.0) + t62 / f64x8::splat(8.0), f64x8::splat(0.704) - t53 + t69));
            let t72 = t28 * t71;
            let t73 = ((f64x8::splat(30.0)).sqrt());
            let t74 = t28 * t73;
            let t75 = ((t71).sqrt());
            let t76 = t28 * t28;
            let t81 = f64x8::splat(0.6018478308354863) * t76 - f64x8::splat(0.0206514);
            let t82 = t71 - f64x8::splat(1.0);
            let t86 = (simd::ln(f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82 + ((((f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82) * (f64x8::splat(0.3910293204892512) / t76 / t28 * t73 * t81 * t82)) + f64x8::splat(1.0)).sqrt())));
            let t90 = f64x8::splat(1.0) + f64x8::splat(0.14163895778062927) * t74 * t75 * t86;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = t72 * t91 + t27;
            let t97 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t93));
            let tzk0 = f64x8::splat(2.0) * t97;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
