//! GGA_K_TFLW exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_tflw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_tflw_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_lambda: f64,
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
        let t32 = param_lambda * sigma0;
        let t33 = rho0 * rho0;
        let t34 = pow_1_3(rho0);
        let t35 = t34 * t34;
        let t37 = 1.0 / t35 / t33;
        let t38 = M_CBRT6;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t47 = param_gamma + 5.0 / 72.0 * t32 * t37 * t38 * t43;
        let t51 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t47);
        let t52 = rho1 <= dens_threshold;
        let t53 = -t17;
        let t55 = piecewise5(t15, t12, t11, t16, t53 * t8);
        let t56 = 1.0 + t55;
        let t57 = t56 <= zeta_threshold;
        let t58 = pow_1_3(t56);
        let t59 = t58 * t58;
        let t61 = piecewise3(t57, t24, t59 * t56);
        let t62 = t61 * t30;
        let t63 = param_lambda * sigma2;
        let t64 = rho1 * rho1;
        let t65 = pow_1_3(rho1);
        let t66 = t65 * t65;
        let t68 = 1.0 / t66 / t64;
        let t73 = param_gamma + 5.0 / 72.0 * t63 * t68 * t38 * t43;
        let t77 = piecewise3(t52, 0.0, 3.0 / 20.0 * t6 * t62 * t73);
        let tzk0 = t51 + t77;
        zk[ip] += tzk0;
    }
}
