//! GGA_K_LLP exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_llp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_llp_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
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
        let t32 = param_beta * t3;
        let t34 = pow_1_3(1.0 / M_PI);
        let t35 = 1.0 / t34;
        let t36 = t32 * t35;
        let t37 = M_CBRT4;
        let t38 = t37 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = pow_1_3(rho0);
        let t41 = t40 * t40;
        let t43 = 1.0 / t41 / t39;
        let t44 = param_gamma * param_beta;
        let t45 = rmath::sqrt(sigma0);
        let t47 = 1.0 / t40 / rho0;
        let t48 = t45 * t47;
        let t49 = rmath::ln(t48 + rmath::sqrt(t48 * t48 + 1.0));
        let t52 = 1.0 + t44 * t48 * t49;
        let t53 = 1.0 / t52;
        let t58 = 1.0 + 2.0 / 9.0 * t36 * t38 * t43 * t53;
        let t62 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t58);
        let t63 = rho1 <= dens_threshold;
        let t64 = -t17;
        let t66 = piecewise5(t15, t12, t11, t16, t64 * t8);
        let t67 = 1.0 + t66;
        let t68 = t67 <= zeta_threshold;
        let t69 = pow_1_3(t67);
        let t70 = t69 * t69;
        let t72 = piecewise3(t68, t24, t70 * t67);
        let t73 = t72 * t30;
        let t74 = t37 * sigma2;
        let t75 = rho1 * rho1;
        let t76 = pow_1_3(rho1);
        let t77 = t76 * t76;
        let t79 = 1.0 / t77 / t75;
        let t80 = rmath::sqrt(sigma2);
        let t82 = 1.0 / t76 / rho1;
        let t83 = t80 * t82;
        let t84 = rmath::ln(t83 + rmath::sqrt(t83 * t83 + 1.0));
        let t87 = 1.0 + t44 * t83 * t84;
        let t88 = 1.0 / t87;
        let t93 = 1.0 + 2.0 / 9.0 * t36 * t74 * t79 * t88;
        let t97 = piecewise3(t63, 0.0, 3.0 / 20.0 * t6 * t73 * t93);
        let tzk0 = t62 + t97;
        zk[ip] += tzk0;
    }
}
