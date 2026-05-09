//! GGA_C_PBE exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 70 shared lines across all orders.
//! Delta: 70 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_pbe_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_BB: f64,
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (70 lines) ---
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
        let t60 = t59 * t59;
        let t61 = t60 * t59;
        let t62 = param_gamma * t61;
        let t63 = rho[ip] * rho[ip];
        let t65 = 1.0 / t7 / t63;
        let t68 = 1.0 / t60;
        let t70 = 1.0 / t3;
        let t72 = t68 * t18 * t70 * t5;
        let t75 = param_BB * param_beta;
        let t76 = 1.0 / param_gamma;
        let t79 = 1.0 / t61;
        let t81 = f64::exp(-(-t32 + t57) * t76 * t79);
        let t82 = t81 - 1.0;
        let t83 = 1.0 / t82;
        let t84 = t76 * t83;
        let t85 = sigma[ip] * sigma[ip];
        let t87 = t75 * t84 * t85;
        let t88 = t63 * t63;
        let t90 = 1.0 / t21 / t88;
        let t91 = t39 * t39;
        let t92 = t90 * t91;
        let t93 = t60 * t60;
        let t94 = 1.0 / t93;
        let t95 = t92 * t94;
        let t96 = 1.0 / t19;
        let t97 = t1 * t96;
        let t98 = t97 * t6;
        let t99 = t95 * t98;
        let t102 = sigma[ip] * t65 * t39 * t72 / 96.0 + t87 * t99 / 3072.0;
        let t103 = param_beta * t102;
        let t104 = param_beta * t76;
        let t107 = t104 * t83 * t102 + 1.0;
        let t108 = 1.0 / t107;
        let t109 = t76 * t108;
        let t111 = t103 * t109 + 1.0;
        let t112 = f64::ln(t111);
        let t113 = t62 * t112;
        let tzk0 = -t32 + t57 + t113;
        zk[ip] += tzk0;
    }
}
