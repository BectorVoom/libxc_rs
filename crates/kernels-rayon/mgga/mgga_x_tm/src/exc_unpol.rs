//! MGGA_X_TM exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`
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
pub fn mgga_x_tm_exc_unpol(
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
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = t22 * t23 / f64x8::splat(8.0);
            let t26 = (t25).simd_lt(f64x8::splat(1.0));
            let t27 = ((t26).select(t25, f64x8::splat(1.0)));
            let t28 = t27 * t27;
            let t29 = t28 * t27;
            let t31 = t28 + f64x8::splat(3.0) * t29;
            let t32 = f64x8::splat(1.0) + t29;
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t31 * t34;
            let t36 = f64x8::splat(M_CBRT6);
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t42 = f64x8::splat(M_CBRT2);
            let t43 = t42 * t42;
            let t44 = v_sigma * t43;
            let t45 = v_rho * v_rho;
            let t46 = t19 * t19;
            let t48 = f64x8::splat(1.0) / t46 / t45;
            let t49 = t44 * t48;
            let t50 = t41 * t49;
            let t52 = t36 * t36;
            let t54 = f64x8::splat(1.0) / t38 / t37;
            let t55 = t52 * t54;
            let t56 = v_sigma * v_sigma;
            let t57 = t56 * t42;
            let t58 = t45 * t45;
            let t59 = t58 * v_rho;
            let t61 = f64x8::splat(1.0) / t19 / t59;
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t50 + f64x8::splat(0.00537989809245259) * t55 * t57 * t61;
            let t66 = (simd::pow(t65, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t69 = v_tau * t43;
            let t71 = f64x8::splat(1.0) / t46 / v_rho;
            let t72 = t69 * t71;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t72 + f64x8::splat(0.256337604) * t52 * t39 + f64x8::splat(0.011867481666666667) * t49) * t36 * t40;
            let t82 = t66 * t66;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) / t66 + f64x8::splat(7.0) / f64x8::splat(9.0) * t81 * t83;
            let t88 = f64x8::splat(1.0) - t35;
            let t91 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t50) * t36;
            let t92 = t91 * t40;
            let t101 = (t72 - t49 / f64x8::splat(8.0)) * t36 * t40 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t50 / f64x8::splat(36.0);
            let t102 = t101 * t101;
            let t104 = t101 * t27;
            let t105 = f64x8::splat(1.0) - t27;
            let t108 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t92 * t49 + f64x8::splat(292.0) / f64x8::splat(405.0) * t102 - f64x8::splat(146.0) / f64x8::splat(135.0) * t104 * t105;
            let t109 = (simd::pow(t108, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t111 = t88 * t109 + t35 * t86;
            let t115 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t111));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
