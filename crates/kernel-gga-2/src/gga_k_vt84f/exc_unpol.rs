//! GGA_K_VT84F exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 81 shared lines across all orders.
//! Delta: 81 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_vt84f_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (81 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = t24 * t24;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t29 = t25 / t27;
        let t30 = f64::sqrt(sigma[ip]);
        let t31 = M_CBRT2;
        let t32 = t30 * t31;
        let t34 = 1.0 / t21 / rho[ip];
        let t37 = t29 * t32 * t34 / 12.0;
        let t38 = f64::sqrt(f64::EPSILON);
        let t39 = t37 <= t38;
        let t41 = (-param_mu + param_alpha + 5.0 / 3.0) * t24;
        let t42 = t27 * t27;
        let t43 = 1.0 / t42;
        let t44 = t41 * t43;
        let t45 = t31 * t31;
        let t46 = sigma[ip] * t45;
        let t47 = rho[ip] * rho[ip];
        let t49 = 1.0 / t22 / t47;
        let t53 = param_mu * param_alpha;
        let t54 = param_mu * param_mu;
        let t56 = (t53 + t54 - param_alpha) * t25;
        let t58 = 1.0 / t27 / t26;
        let t59 = t56 * t58;
        let t60 = sigma[ip] * sigma[ip];
        let t61 = t60 * t31;
        let t62 = t47 * t47;
        let t63 = t62 * rho[ip];
        let t65 = 1.0 / t21 / t63;
        let t69 = param_alpha * param_alpha;
        let t71 = param_mu * t69 / 2.0;
        let t74 = t69 / 2.0;
        let t76 = t26 * t26;
        let t78 = (-t71 - (t53 + t54) * param_mu - t74) / t76;
        let t79 = t60 * sigma[ip];
        let t80 = t62 * t62;
        let t81 = 1.0 / t80;
        let t85 = t69 * param_alpha;
        let t89 = t54 * param_mu;
        let t93 = (param_mu * t85 / 6.0 - (-param_alpha * t54 - t71 - t89) * param_mu + t74) * t24;
        let t95 = 1.0 / t42 / t76;
        let t96 = t93 * t95;
        let t97 = t60 * t60;
        let t98 = t97 * t45;
        let t99 = t80 * t47;
        let t101 = 1.0 / t22 / t99;
        let t106 = t38 < t37;
        let t107 = piecewise3(t106, t37, t38);
        let t108 = t107 * t107;
        let t109 = param_mu * t108;
        let t110 = param_alpha * t108;
        let t111 = f64::exp(-t110);
        let t112 = 1.0 + t109;
        let t113 = 1.0 / t112;
        let t114 = t111 * t113;
        let t116 = t108 * t108;
        let t118 = f64::exp(-param_alpha * t116);
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t108;
        let t121 = t120 - 1.0;
        let t125 = piecewise3(t39, 1.0 + t44 * t46 * t49 / 24.0 + t59 * t61 * t65 / 288.0 + t78 * t79 * t81 / 576.0 + t96 * t98 * t101 / 13824.0, 1.0 - t109 * t114 + t119 * t121 + 5.0 / 3.0 * t108);
        let t129 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t125);
        let tzk0 = 2.0 * t129;
        zk[ip] += tzk0;
    }
}
