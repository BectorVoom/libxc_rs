//! GGA_C_SCAN_E0 exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 64 shared lines across all orders.
//! Delta: 64 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_scan_e0_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (64 lines) ---
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
        let t58 = f64::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t62 = t59 / t60;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.25e-1 * t10;
        let t70 = 1.0 + 0.4445e-1 * t10;
        let t71 = 1.0 / t70;
        let t72 = t68 * t71;
        let t73 = 1.0 / t59;
        let t76 = 1.0 / t66;
        let t77 = t60 * t76;
        let t79 = f64::exp(-(-t32 + t57) * t73 * t77);
        let t80 = t79 - 1.0;
        let t81 = 1.0 / t80;
        let t82 = t73 * t81;
        let t83 = t82 * sigma[ip];
        let t84 = t72 * t83;
        let t85 = rho[ip] * rho[ip];
        let t87 = 1.0 / t7 / t85;
        let t88 = t87 * t39;
        let t89 = 1.0 / t65;
        let t91 = 1.0 / t3;
        let t93 = t18 * t91 * t5;
        let t97 = 1.0 + 0.27439371595564631661e-1 * t84 * t88 * t89 * t93;
        let t98 = pow_1_4(t97);
        let t100 = 1.0 - 1.0 / t98;
        let t103 = 1.0 + 1.0 * t100 * t80;
        let t104 = f64::ln(t103);
        let t106 = t62 * t66 * t104;
        let tzk0 = -t32 + t57 + t106;
        zk[ip] += tzk0;
    }
}
