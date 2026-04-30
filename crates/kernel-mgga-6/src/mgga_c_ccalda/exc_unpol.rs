//! MGGA_C_CCALDA exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 51 shared lines across all orders.
//! Delta: 51 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_c_ccalda_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (51 lines) ---
        let t2 = 1.0 + param_c;
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / rho[ip];
        let t8 = rho[ip] * rho[ip];
        let t10 = 1.0 / t4 / t8;
        let t13 = tau[ip] * t6 - sigma[ip] * t10 / 8.0;
        let t14 = t2 * t13;
        let t15 = M_CBRT6;
        let t16 = t14 * t15;
        let t17 = M_PI * M_PI;
        let t18 = pow_1_3(t17);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t21 = M_CBRT2;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t26 = t15 * t20 * t22;
        let t29 = 1.0 + 5.0 / 9.0 * param_c * t13 * t26;
        let t30 = 1.0 / t29;
        let t31 = M_CBRT3;
        let t32 = 1.0 / M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t39 = t34 * t36 / t3;
        let t41 = 1.0 + 0.53425e-1 * t39;
        let t42 = f64::sqrt(t39);
        let t45 = pow_3_2(t39);
        let t47 = t31 * t31;
        let t48 = t33 * t33;
        let t49 = t47 * t48;
        let t52 = t49 * t35 / t4;
        let t54 = 0.379785e1 * t42 + 0.8969e0 * t39 + 0.204775e0 * t45 + 0.123235e0 * t52;
        let t57 = 1.0 + 0.16081979498692535067e2 / t54;
        let t58 = f64::ln(t57);
        let t62 = pow_1_3(zeta_threshold);
        let t64 = piecewise3(1.0 <= zeta_threshold, t62 * zeta_threshold, 1.0);
        let t70 = (2.0 * t64 - 2.0) / (2.0 * t21 - 2.0);
        let t72 = 1.0 + 0.278125e-1 * t39;
        let t77 = 0.51785e1 * t42 + 0.905775e0 * t39 + 0.1100325e0 * t45 + 0.1241775e0 * t52;
        let t80 = 1.0 + 0.29608749977793437516e2 / t77;
        let t81 = f64::ln(t80);
        let t85 = -0.621814e-1 * t41 * t58 + 0.19751673498613801407e-1 * t70 * t72 * t81;
        let t87 = t23 * t30 * t85;
        let t89 = 5.0 / 9.0 * t16 * t87;
        let t90 = t23 * t30;
        let t93 = 1.0 - 5.0 / 9.0 * t16 * t90;
        let t94 = t93 * t85;
        let tzk0 = t89 + t94;
        zk[ip] += tzk0;
    }
}
