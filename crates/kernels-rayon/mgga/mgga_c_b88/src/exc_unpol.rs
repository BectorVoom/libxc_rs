//! MGGA_C_B88 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_b88_exc_unpol(
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
            let t5 = t4 * t4;
            let t6 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t7 = (simd::cbrt(t6));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t5 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t9 * t10;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = ((t13).select(t14, (t13).select(-t14, f64x8::splat(0.0))));
            let t17 = f64x8::splat(1.0) + t16;
            let t18 = t17 * v_rho;
            let t19 = (simd::cbrt(t18));
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t12 * t20;
            let t22 = t12 * t12;
            let t23 = v_sigma * t22;
            let t24 = v_rho * v_rho;
            let t25 = (simd::cbrt(v_rho));
            let t26 = t25 * t25;
            let t28 = f64x8::splat(1.0) / t26 / t24;
            let t29 = t23 * t28;
            let t31 = f64x8::splat(1.0) + f64x8::splat(0.007) * t29;
            let t32 = (simd::pow(t31, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t33 = t32 * t32;
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.0008333333333333334) * t11 * t23 * t28 * t35;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t11 * t21 * t41;
            let t45 = ((t3).select(f64x8::splat(0.0), t43 / f64x8::splat(9.0)));
            let t46 = v_rho * t45;
            let t47 = f64x8::splat(1.26) * t45;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (simd::ln(t48));
            let t50 = t47 - t49;
            let t52 = f64x8::splat(0.252) * t46 * t50;
            let t53 = t17 * t17;
            let t54 = (simd::cbrt(t17));
            let t55 = t54 * t54;
            let t56 = t55 * t53;
            let t57 = t56 * t22;
            let t58 = t26 * v_rho;
            let t59 = v_tau * t22;
            let t64 = f64x8::splat(2.0) * t59 / t58 - t29 / f64x8::splat(4.0);
            let t66 = t58 * t64 * t5;
            let t67 = t57 * t66;
            let t69 = f64x8::splat(1.0) / t7 / t6;
            let t70 = t69 * t10;
            let t72 = f64x8::splat(1.0) / t19 / t18;
            let t73 = t40 * t40;
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t72 * t75;
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.10666666666666667) * t43;
            let t79 = (simd::ln(t78));
            let t80 = t79 * t4;
            let t81 = t80 * t7;
            let t82 = t10 * t10;
            let t83 = t82 * t22;
            let t84 = t19 * t40;
            let t85 = t83 * t84;
            let t88 = f64x8::splat(1.0) - f64x8::splat(0.390625) * t81 * t85;
            let t90 = t70 * t76 * t88;
            let t93 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.0001864135111111111) * t67 * t90));
            let t94 = f64x8::splat(2.0) * t93;
            let tzk0 = -t52 + t94;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
