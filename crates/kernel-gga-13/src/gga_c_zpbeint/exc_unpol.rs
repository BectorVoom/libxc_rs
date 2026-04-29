//! GGA_C_ZPBEINT exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 80 shared lines across all orders.
//! Delta: 80 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_zpbeint_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (80 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t32 = 0.621814e-1 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t57 = 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t58 = t34 * t34;
        let t59 = piecewise3(t33, t58, 1.0);
        let t60 = f64::sqrt(sigma[ip]);
        let t61 = t60 * sigma[ip];
        let t62 = param_alpha * t61;
        let t63 = rho[ip] * rho[ip];
        let t64 = t63 * t63;
        let t65 = 1.0 / t64;
        let t66 = t59 * t59;
        let t67 = t66 * t59;
        let t68 = 1.0 / t67;
        let t71 = 1.0 / t13 / t10;
        let t75 = f64::powf(t59, t62 * t65 * t68 * t71 / 16.0);
        let t76 = f64::ln(2.0);
        let t77 = 1.0 - t76;
        let t78 = t75 * t77;
        let t79 = M_PI * M_PI;
        let t80 = 1.0 / t79;
        let t81 = t80 * t67;
        let t83 = 1.0 / t7 / t63;
        let t86 = 1.0 / t66;
        let t88 = 1.0 / t3;
        let t90 = t86 * t18 * t88 * t5;
        let t93 = 1.0 / t77;
        let t94 = param_beta * t93;
        let t99 = f64::exp(-(-t32 + t57) * t93 * t79 * t68);
        let t100 = t99 - 1.0;
        let t101 = 1.0 / t100;
        let t102 = t79 * t101;
        let t103 = sigma[ip] * sigma[ip];
        let t105 = t94 * t102 * t103;
        let t107 = 1.0 / t21 / t64;
        let t108 = t39 * t39;
        let t109 = t107 * t108;
        let t110 = t66 * t66;
        let t111 = 1.0 / t110;
        let t112 = t109 * t111;
        let t113 = 1.0 / t19;
        let t114 = t1 * t113;
        let t115 = t114 * t6;
        let t116 = t112 * t115;
        let t119 = sigma[ip] * t83 * t39 * t90 / 96.0 + t105 * t116 / 3072.0;
        let t120 = param_beta * t119;
        let t124 = t94 * t102 * t119 + 1.0;
        let t125 = 1.0 / t124;
        let t126 = t93 * t79 * t125;
        let t128 = t120 * t126 + 1.0;
        let t129 = f64::ln(t128);
        let t131 = t78 * t81 * t129;
        let tzk0 = -t32 + t57 + t131;
        zk[ip] += tzk0;
    }
}
