//! GGA_C_AM05 vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 45 shared lines across all orders.
//! Delta: 47 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_am05_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (45 lines) ---
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
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t58 = -0.621814e-1 * t12 * t30 + 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t59 = piecewise3(t33, zeta_threshold, 1.0);
        let t60 = t58 * t59;
        let t61 = M_CBRT6;
        let t62 = param_alpha * t61;
        let t63 = M_PI * M_PI;
        let t64 = pow_1_3(t63);
        let t65 = t64 * t64;
        let t66 = 1.0 / t65;
        let t68 = t39 * t39;
        let t69 = sigma[ip] * t68;
        let t70 = rho[ip] * rho[ip];
        let t72 = 1.0 / t21 / t70;
        let t76 = 1.0 + t62 * t66 * t69 * t72 / 24.0;
        let t77 = 1.0 / t76;
        let t80 = t77 + param_gamma * (1.0 - t77);
        let tzk0 = t60 * t80;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (47 lines) ---
        let t82 = 1.0 / t7 / rho[ip];
        let t83 = t6 * t82;
        let t87 = t26 * t26;
        let t88 = 1.0 / t87;
        let t89 = t12 * t88;
        let t91 = 1.0 / t13 * t1;
        let t92 = t3 * t6;
        let t93 = t92 * t82;
        let t94 = t91 * t93;
        let t96 = t4 * t83;
        let t98 = f64::sqrt(t10);
        let t99 = t98 * t1;
        let t100 = t99 * t93;
        let t105 = t20 * t5 / t21 / rho[ip];
        let t107 = -0.632975e0 * t94 - 0.29896666666666666667e0 * t96 - 0.1023875e0 * t100 - 0.82156666666666666667e-1 * t105;
        let t108 = 1.0 / t29;
        let t109 = t107 * t108;
        let t112 = t43 * t1;
        let t117 = t43 * t45;
        let t118 = t50 * t50;
        let t119 = 1.0 / t118;
        let t124 = -0.86308333333333333334e0 * t94 - 0.301925e0 * t96 - 0.5501625e-1 * t100 - 0.82785e-1 * t105;
        let t126 = 1.0 / t53;
        let t127 = t119 * t124 * t126;
        let t130 = 0.11073470983333333333e-2 * t4 * t83 * t30 + 1.0 * t89 * t109 - 0.18311447306006545054e-3 * t112 * t92 * t82 * t54 - 0.5848223622634646207e0 * t117 * t127;
        let t131 = rho[ip] * t130;
        let t132 = t59 * t80;
        let t134 = rho[ip] * t58;
        let t135 = t76 * t76;
        let t136 = 1.0 / t135;
        let t138 = t136 * param_alpha * t61;
        let t139 = t66 * sigma[ip];
        let t140 = t70 * rho[ip];
        let t142 = 1.0 / t21 / t140;
        let t143 = t68 * t142;
        let t144 = t139 * t143;
        let t146 = param_gamma * t136;
        let t147 = t146 * t62;
        let t150 = t138 * t144 / 9.0 - t147 * t144 / 9.0;
        let t151 = t59 * t150;
        let tvrho0 = t131 * t132 + t134 * t151 + tzk0;
        vrho[ip] += tvrho0;
        let t153 = t66 * t68;
        let t156 = t146 * param_alpha;
        let t157 = t61 * t66;
        let t162 = t156 * t157 * t68 * t72 / 24.0 - t138 * t153 * t72 / 24.0;
        let t163 = t59 * t162;
        let tvsigma0 = t134 * t163;
        vsigma[ip] += tvsigma0;
    }
}
