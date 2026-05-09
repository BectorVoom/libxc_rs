//! GGA_X_AK13 exc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 57 shared lines across all orders.
//! Delta: 57 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ak13_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_B1: f64,
    param_B2: f64,
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
        let t31 = M_PI * M_PI;
        let t32 = pow_1_3(t31);
        let t33 = 1.0 / t32;
        let t34 = param_B1 * t29 * t33;
        let t35 = f64::sqrt(sigma0);
        let t36 = pow_1_3(rho0);
        let t38 = 1.0 / t36 / rho0;
        let t39 = t35 * t38;
        let t40 = t29 * t33;
        let t43 = 1.0 + t40 * t39 / 12.0;
        let t44 = f64::ln(t43);
        let t49 = param_B2 * t29 * t33;
        let t50 = 1.0 + t44;
        let t51 = f64::ln(t50);
        let t55 = 1.0 + t34 * t39 * t44 / 12.0 + t49 * t39 * t51 / 12.0;
        let t59 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t55);
        let t60 = rho1 <= dens_threshold;
        let t61 = -t16;
        let t63 = piecewise5(t14, t11, t10, t15, t61 * t7);
        let t64 = 1.0 + t63;
        let t65 = t64 <= zeta_threshold;
        let t66 = pow_1_3(t64);
        let t68 = piecewise3(t65, t22, t66 * t64);
        let t69 = t68 * t26;
        let t70 = f64::sqrt(sigma2);
        let t71 = pow_1_3(rho1);
        let t73 = 1.0 / t71 / rho1;
        let t74 = t70 * t73;
        let t77 = 1.0 + t40 * t74 / 12.0;
        let t78 = f64::ln(t77);
        let t82 = 1.0 + t78;
        let t83 = f64::ln(t82);
        let t87 = 1.0 + t34 * t74 * t78 / 12.0 + t49 * t74 * t83 / 12.0;
        let t91 = piecewise3(t60, 0.0, -3.0 / 8.0 * t5 * t69 * t87);
        let tzk0 = t59 + t91;
        zk[ip] += tzk0;
    }
}
