//! GGA_X_FD_LB94 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 57 shared lines across all orders.
//! Delta: 57 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::integrate::{xc_integrate_func0, xc_integrate_func1};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_fd_lb94_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        // --- shared preamble (57 lines) ---
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
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t38 = t34 * t37;
        let t40 = t33 * t38 / 12.0;
        let t41 = xc_integrate_func0(t40, param_beta);
        let t42 = f64::ln(t40);
        let t44 = xc_integrate_func1(t40, param_beta);
        let t45 = t41 * t42 - t44;
        let t49 = 1.0 - t33 * t38 * t45 / 12.0;
        let t53 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t49);
        let t54 = rho1 <= dens_threshold;
        let t55 = -t16;
        let t57 = piecewise5(t14, t11, t10, t15, t55 * t7);
        let t58 = 1.0 + t57;
        let t59 = t58 <= zeta_threshold;
        let t60 = pow_1_3(t58);
        let t62 = piecewise3(t59, t22, t60 * t58);
        let t63 = t62 * t26;
        let t64 = f64::sqrt(sigma2);
        let t65 = pow_1_3(rho1);
        let t67 = 1.0 / t65 / rho1;
        let t68 = t64 * t67;
        let t70 = t33 * t68 / 12.0;
        let t71 = xc_integrate_func0(t70, param_beta);
        let t72 = f64::ln(t70);
        let t74 = xc_integrate_func1(t70, param_beta);
        let t75 = t71 * t72 - t74;
        let t79 = 1.0 - t33 * t68 * t75 / 12.0;
        let t83 = piecewise3(t54, 0.0, -3.0 / 8.0 * t5 * t63 * t79);
        let tzk0 = t53 + t83;
        zk[ip] += tzk0;
    }
}
