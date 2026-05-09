//! GGA_X_OPTX exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 55 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_optx_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_gamma: f64,
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
        // --- shared preamble (55 lines) ---
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
        let t28 = param_gamma * param_gamma;
        let t29 = param_b * t28;
        let t30 = sigma0 * sigma0;
        let t31 = rho0 * rho0;
        let t32 = t31 * t31;
        let t33 = t32 * rho0;
        let t34 = pow_1_3(rho0);
        let t36 = 1.0 / t34 / t33;
        let t39 = t34 * t34;
        let t43 = 1.0 + param_gamma * sigma0 / t39 / t31;
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t48 = t29 * t30 * t36 * t45 + param_a;
        let t52 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t48);
        let t53 = rho1 <= dens_threshold;
        let t54 = -t16;
        let t56 = piecewise5(t14, t11, t10, t15, t54 * t7);
        let t57 = 1.0 + t56;
        let t58 = t57 <= zeta_threshold;
        let t59 = pow_1_3(t57);
        let t61 = piecewise3(t58, t22, t59 * t57);
        let t62 = t61 * t26;
        let t63 = sigma2 * sigma2;
        let t64 = rho1 * rho1;
        let t65 = t64 * t64;
        let t66 = t65 * rho1;
        let t67 = pow_1_3(rho1);
        let t69 = 1.0 / t67 / t66;
        let t72 = t67 * t67;
        let t76 = 1.0 + param_gamma * sigma2 / t72 / t64;
        let t77 = t76 * t76;
        let t78 = 1.0 / t77;
        let t81 = t29 * t63 * t69 * t78 + param_a;
        let t85 = piecewise3(t53, 0.0, -3.0 / 8.0 * t5 * t62 * t81);
        let tzk0 = t52 + t85;
        zk[ip] += tzk0;
    }
}
