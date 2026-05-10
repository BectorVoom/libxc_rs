//! GGA_C_LYPR exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 55 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_PI};
use libxc_kernel_math::erf::{erfc_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_lypr_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_m1: f64,
    param_m2: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (55 lines) ---
        let t2 = pow_1_3(rho[ip]);
        let t3 = 1.0 / t2;
        let t5 = erfc_approx(param_m1 * param_omega * t3);
        let t7 = param_d * t3 + 1.0;
        let t8 = 1.0 / t7;
        let t10 = param_m2 * param_omega;
        let t12 = erfc_approx(t10 * t3);
        let t13 = t12 * param_b;
        let t15 = f64::exp(-param_c * t3);
        let t16 = t15 * t8;
        let t17 = rho[ip] * rho[ip];
        let t18 = t2 * t2;
        let t20 = 1.0 / t18 / t17;
        let t21 = sigma[ip] * t20;
        let t23 = param_d * t8 + param_c;
        let t24 = t23 * t3;
        let t26 = -1.0 / 72.0 - 7.0 / 72.0 * t24;
        let t28 = M_CBRT3;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = zeta_threshold * zeta_threshold;
        let t36 = pow_1_3(zeta_threshold);
        let t37 = t36 * t36;
        let t39 = piecewise3(t34, t37 * t35, 1.0);
        let t43 = 5.0 / 2.0 - t24 / 18.0;
        let t44 = t43 * sigma[ip];
        let t45 = t20 * t39;
        let t48 = t24 - 11.0;
        let t49 = t48 * sigma[ip];
        let t52 = piecewise3(t34, t37 * t35 * zeta_threshold, 1.0);
        let t53 = t20 * t52;
        let t56 = M_CBRT2;
        let t57 = t56 * t56;
        let t58 = sigma[ip] * t57;
        let t61 = piecewise3(t34, t35, 1.0);
        let t62 = t61 * sigma[ip];
        let t64 = t57 * t20 * t39;
        let t70 = -t21 * t26 - 3.0 / 10.0 * t29 * t32 * t39 + t44 * t45 / 8.0 + t49 * t53 / 144.0 - t56 * (4.0 / 3.0 * t58 * t45 - t62 * t64 / 2.0) / 8.0;
        let t71 = t16 * t70;
        let t73 = param_b * t15;
        let t74 = f64::sqrt(M_PI);
        let t75 = 1.0 / t74;
        let t76 = t8 * t75;
        let t77 = t73 * t76;
        let t78 = param_m2 * param_m2;
        let t79 = param_omega * param_omega;
        let t81 = 1.0 / t18;
        let t83 = f64::exp(-t78 * t79 * t81);
        let t84 = t17 * rho[ip];
        let t85 = 1.0 / t84;
        let t86 = t83 * t85;
        let tzk0 = param_a * (-t5 * t8 + t13 * t71 + 7.0 / 36.0 * t77 * t10 * t86 * sigma[ip]);
        zk[ip] += tzk0;
    }
}
