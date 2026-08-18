//! GGA_X_LB vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lb_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..vrho.len() / 2 {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t9 = param_alpha * t1 * t4 * t6 / 2.0;
        let t10 = f64::sqrt(sigma0);
        let t11 = pow_1_3(rho0);
        let t13 = 1.0 / t11 / rho0;
        let t14 = t10 * t13;
        let t15 = t14 < 300.0;
        let t16 = param_beta * sigma0;
        let t17 = rho0 * rho0;
        let t18 = t11 * t11;
        let t20 = 1.0 / t18 / t17;
        let t21 = param_beta * t10;
        let t23 = param_gamma * t10 * t13;
        let t24 = f64::ln(t23 + f64::sqrt(t23 * t23 + 1.0));
        let t25 = t13 * t24;
        let t28 = 3.0 * t21 * t25 + 1.0;
        let t29 = 1.0 / t28;
        let t33 = f64::ln(2.0 * t23);
        let t34 = 1.0 / t33;
        let t37 = piecewise3(t15, t16 * t20 * t29, t14 * t34 / 3.0);
        let t38 = -t9 - t37;
        let tvrho0 = t38 * t11;
        vrho[ip * 2] += tvrho0;
        let t39 = f64::sqrt(sigma2);
        let t40 = pow_1_3(rho1);
        let t42 = 1.0 / t40 / rho1;
        let t43 = t39 * t42;
        let t44 = t43 < 300.0;
        let t45 = param_beta * sigma2;
        let t46 = rho1 * rho1;
        let t47 = t40 * t40;
        let t49 = 1.0 / t47 / t46;
        let t50 = param_beta * t39;
        let t52 = param_gamma * t39 * t42;
        let t53 = f64::ln(t52 + f64::sqrt(t52 * t52 + 1.0));
        let t54 = t42 * t53;
        let t57 = 3.0 * t50 * t54 + 1.0;
        let t58 = 1.0 / t57;
        let t62 = f64::ln(2.0 * t52);
        let t63 = 1.0 / t62;
        let t66 = piecewise3(t44, t45 * t49 * t58, t43 * t63 / 3.0);
        let t67 = -t9 - t66;
        let tvrho1 = t67 * t40;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
