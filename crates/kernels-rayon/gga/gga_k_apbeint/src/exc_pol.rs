//! GGA_K_APBEINT exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_apbeint_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
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
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = param_muPBE - param_muGE;
        let t34 = M_CBRT6;
        let t35 = t32 * param_alpha * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t39 * sigma0;
        let t41 = rho0 * rho0;
        let t42 = pow_1_3(rho0);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t46 = param_alpha * t34;
        let t47 = t40 * t45;
        let t50 = 1.0 + t46 * t47 / 24.0;
        let t51 = 1.0 / t50;
        let t57 = (param_muGE + t35 * t40 * t45 * t51 / 24.0) * t34;
        let t60 = param_kappa + t57 * t47 / 24.0;
        let t65 = 1.0 + param_kappa * (1.0 - param_kappa / t60);
        let t69 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t17;
        let t73 = piecewise5(t15, t12, t11, t16, t71 * t8);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3(t74);
        let t77 = t76 * t76;
        let t79 = piecewise3(t75, t24, t77 * t74);
        let t80 = t79 * t30;
        let t81 = t39 * sigma2;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / t82;
        let t87 = t81 * t86;
        let t90 = 1.0 + t46 * t87 / 24.0;
        let t91 = 1.0 / t90;
        let t97 = (param_muGE + t35 * t81 * t86 * t91 / 24.0) * t34;
        let t100 = param_kappa + t97 * t87 / 24.0;
        let t105 = 1.0 + param_kappa * (1.0 - param_kappa / t100);
        let t109 = piecewise3(t70, 0.0, 3.0 / 20.0 * t6 * t80 * t105);
        let tzk0 = t69 + t109;
        zk[ip] += tzk0;
    }
}
