//! MGGA_K_PC07 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 86 shared lines across all orders.
//! Delta: 86 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_k_pc07_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (86 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = M_CBRTPI;
        let t8 = t5 * t6 * M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t16 = t15 * t15;
        let t18 = pow_1_3(t13);
        let t19 = t18 * t18;
        let t21 = piecewise3(t13 <= zeta_threshold, t16 * zeta_threshold, t19 * t13);
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t24 = t21 * t23;
        let t25 = M_CBRT6;
        let t26 = M_PI * M_PI;
        let t27 = pow_1_3(t26);
        let t28 = t27 * t27;
        let t29 = 1.0 / t28;
        let t30 = t25 * t29;
        let t31 = M_CBRT2;
        let t32 = t31 * t31;
        let t33 = sigma[ip] * t32;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t23 / t34;
        let t38 = t30 * t33 * t36;
        let t39 = 5.0 / 72.0 * t38;
        let t41 = lapl[ip] * t32;
        let t43 = 1.0 / t23 / rho[ip];
        let t47 = t25 * t25;
        let t49 = 1.0 / t27 / t26;
        let t50 = t47 * t49;
        let t51 = lapl[ip] * lapl[ip];
        let t52 = t51 * t31;
        let t53 = t34 * rho[ip];
        let t55 = 1.0 / t22 / t53;
        let t58 = t50 * t52 * t55 / 2916.0;
        let t59 = t50 * sigma[ip];
        let t60 = t34 * t34;
        let t62 = 1.0 / t22 / t60;
        let t63 = t31 * t62;
        let t64 = t63 * lapl[ip];
        let t66 = t59 * t64 / 2592.0;
        let t67 = sigma[ip] * sigma[ip];
        let t68 = t67 * t31;
        let t69 = t60 * rho[ip];
        let t71 = 1.0 / t22 / t69;
        let t74 = t50 * t68 * t71 / 8748.0;
        let t75 = 1.0 + 5.0 / 648.0 * t38 + 5.0 / 54.0 * t30 * t41 * t43 + t58 - t66 + t74;
        let t76 = t58 - t66 + t74;
        let t77 = t76 * t76;
        let t78 = 1.0 + t39;
        let t79 = t78 * t78;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let t83 = f64::sqrt(t82);
        let t84 = 1.0 / t83;
        let t86 = t75 * t84 - t39;
        let t87 = param_a / 40.0;
        let t88 = t86 <= t87;
        let t89 = 39.0 / 40.0 * param_a;
        let t90 = t89 <= t86;
        let t91 = param_a * param_b;
        let t92 = t86 < t87;
        let t93 = piecewise3(t92, t87, t86);
        let t94 = t93 < t89;
        let t95 = piecewise3(t94, t93, t89);
        let t96 = 1.0 / t95;
        let t98 = f64::exp(-t91 * t96);
        let t99 = param_a - t95;
        let t102 = f64::exp(-param_a / t99);
        let t103 = 1.0 + t102;
        let t104 = f64::powf(t103, param_b);
        let t105 = t98 * t104;
        let t107 = f64::exp(-param_a * t96);
        let t108 = t107 + t102;
        let t109 = f64::powf(t108, param_b);
        let t110 = 1.0 / t109;
        let t111 = t105 * t110;
        let t112 = piecewise5(t88, 0.0, t90, 1.0, t111);
        let t114 = t86 * t112 + t39;
        let t118 = piecewise3(t3, 0.0, 3.0 / 20.0 * t8 * t24 * t114);
        let tzk0 = 2.0 * t118;
        zk[ip] += tzk0;
    }
}
