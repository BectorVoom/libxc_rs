//! GGA_X_DK87 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_dk87.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_dk87_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a1: f64,
    param_alpha: f64,
    param_b1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = 1.0 / M_PI;
        let t29 = M_CBRT6;
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = 1.0 / t33;
        let t35 = t2 * t2;
        let t36 = t34 * t35;
        let t37 = pow_1_3(t28);
        let t38 = 1.0 / t37;
        let t40 = t31 * t36 * t38;
        let t41 = M_CBRT4;
        let t42 = t41 * sigma0;
        let t43 = rho0 * rho0;
        let t44 = pow_1_3(rho0);
        let t45 = t44 * t44;
        let t47 = 1.0 / t45 / t43;
        let t48 = f64::sqrt(sigma0);
        let t52 = f64::powf(t48 / t44 / rho0, param_alpha);
        let t54 = param_a1 * t52 + 1.0;
        let t56 = param_b1 * sigma0;
        let t58 = t56 * t47 + 1.0;
        let t59 = 1.0 / t58;
        let t60 = t47 * t54 * t59;
        let t64 = 1.0 + 7.0 / 11664.0 * t40 * t42 * t60;
        let t68 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t64);
        let t69 = rho1 <= dens_threshold;
        let t70 = -t16;
        let t72 = piecewise5(t14, t11, t10, t15, t70 * t7);
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(t73);
        let t77 = piecewise3(t74, t22, t75 * t73);
        let t78 = t77 * t26;
        let t79 = t41 * sigma2;
        let t80 = rho1 * rho1;
        let t81 = pow_1_3(rho1);
        let t82 = t81 * t81;
        let t84 = 1.0 / t82 / t80;
        let t85 = f64::sqrt(sigma2);
        let t89 = f64::powf(t85 / t81 / rho1, param_alpha);
        let t91 = param_a1 * t89 + 1.0;
        let t93 = param_b1 * sigma2;
        let t95 = t93 * t84 + 1.0;
        let t96 = 1.0 / t95;
        let t97 = t84 * t91 * t96;
        let t101 = 1.0 + 7.0 / 11664.0 * t40 * t79 * t97;
        let t105 = piecewise3(t69, 0.0, -3.0 / 8.0 * t5 * t78 * t101);
        let tzk0 = t68 + t105;
        zk[ip] += tzk0;
    }
}
