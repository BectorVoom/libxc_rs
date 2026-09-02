//! GGA_K_OL2 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_ol2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_ol2_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_bb: f64,
    param_cc: f64,
    param_aa: f64,
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
        let t32 = param_bb * sigma0;
        let t33 = rho0 * rho0;
        let t34 = pow_1_3(rho0);
        let t35 = t34 * t34;
        let t37 = 1.0 / t35 / t33;
        let t40 = rmath::sqrt(sigma0);
        let t41 = param_cc * t40;
        let t43 = 1.0 / t34 / rho0;
        let t44 = M_CBRT2;
        let t47 = 4.0 * t40 * t43 + t44;
        let t48 = 1.0 / t47;
        let t49 = t43 * t48;
        let t51 = param_aa + 0.013888888888888888 * t32 * t37 + t41 * t49;
        let t55 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t51);
        let t56 = rho1 <= dens_threshold;
        let t57 = -t17;
        let t59 = piecewise5(t15, t12, t11, t16, t57 * t8);
        let t60 = 1.0 + t59;
        let t61 = t60 <= zeta_threshold;
        let t62 = pow_1_3(t60);
        let t63 = t62 * t62;
        let t65 = piecewise3(t61, t24, t63 * t60);
        let t66 = t65 * t30;
        let t67 = param_bb * sigma2;
        let t68 = rho1 * rho1;
        let t69 = pow_1_3(rho1);
        let t70 = t69 * t69;
        let t72 = 1.0 / t70 / t68;
        let t75 = rmath::sqrt(sigma2);
        let t76 = param_cc * t75;
        let t78 = 1.0 / t69 / rho1;
        let t81 = 4.0 * t75 * t78 + t44;
        let t82 = 1.0 / t81;
        let t83 = t78 * t82;
        let t85 = param_aa + 0.013888888888888888 * t67 * t72 + t76 * t83;
        let t89 = piecewise3(t56, 0.0, 3.0 / 20.0 * t6 * t66 * t85);
        let tzk0 = t55 + t89;
        zk[ip] += tzk0;
    }
}
