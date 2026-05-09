//! MGGA_X_R4SCAN exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 116 shared lines across all orders.
//! Delta: 116 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_r4scan_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_da4: f64,
    param_dp2: f64,
    param_dp4: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (116 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t22 = 20.0 / 27.0 + 5.0 / 3.0 * param_eta;
        let t23 = M_CBRT6;
        let t24 = t23 * t23;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t25;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = sigma[ip] * sigma[ip];
        let t31 = t29 * t30;
        let t32 = M_CBRT2;
        let t33 = rho[ip] * rho[ip];
        let t34 = t33 * t33;
        let t35 = t34 * rho[ip];
        let t37 = 1.0 / t20 / t35;
        let t38 = t32 * t37;
        let t39 = param_dp2 * param_dp2;
        let t40 = t39 * t39;
        let t41 = 1.0 / t40;
        let t45 = f64::exp(-t31 * t38 * t41 / 288.0);
        let t49 = (-0.162742215233874e0 * t22 * t45 + 10.0 / 81.0) * t23;
        let t50 = t26 * t26;
        let t51 = 1.0 / t50;
        let t52 = t49 * t51;
        let t53 = t32 * t32;
        let t54 = sigma[ip] * t53;
        let t55 = t20 * t20;
        let t57 = 1.0 / t55 / t33;
        let t58 = t54 * t57;
        let t61 = param_k1 + t52 * t58 / 24.0;
        let t65 = param_k1 * (1.0 - param_k1 / t61);
        let t66 = tau[ip] * t53;
        let t67 = t55 * rho[ip];
        let t68 = 1.0 / t67;
        let t71 = t66 * t68 - t58 / 8.0;
        let t74 = param_eta * sigma[ip];
        let t75 = t53 * t57;
        let t78 = 3.0 / 10.0 * t24 * t50 + t74 * t75 / 8.0;
        let t79 = 1.0 / t78;
        let t80 = t71 * t79;
        let t81 = t80 <= 0.0;
        let t82 = 0.0 < t80;
        let t83 = piecewise3(t82, 0.0, t80);
        let t84 = param_c1 * t83;
        let t85 = 1.0 - t83;
        let t86 = 1.0 / t85;
        let t88 = f64::exp(-t84 * t86);
        let t89 = t80 <= 0.25e1;
        let t90 = 0.25e1 < t80;
        let t91 = piecewise3(t90, 0.25e1, t80);
        let t93 = t91 * t91;
        let t95 = t93 * t91;
        let t97 = t93 * t93;
        let t99 = t97 * t91;
        let t101 = t97 * t93;
        let t106 = piecewise3(t90, t80, 0.25e1);
        let t107 = 1.0 - t106;
        let t110 = f64::exp(param_c2 / t107);
        let t112 = piecewise5(t81, t88, t89, 1.0 - 0.667e0 * t91 - 0.4445555e0 * t93 - 0.663086601049e0 * t95 + 0.145129704449e1 * t97 - 0.887998041597e0 * t99 + 0.234528941479e0 * t101 - 0.23185843322e-1 * t97 * t95, -param_d * t110);
        let t113 = 0.174e0 - t65;
        let t116 = t22 * t23;
        let t117 = t116 * t51;
        let t120 = 1.0 - t80;
        let t121 = t120 * t120;
        let t125 = (0.40570770199022687793e-1 - 0.30235468026081006357e0 * param_eta) * t23;
        let t126 = t125 * t51;
        let t133 = pow_2(3.0 / 4.0 * param_eta + 2.0 / 3.0);
        let t138 = pow_2(0.290700106132790123e-2 - 0.27123702538979e0 * param_eta);
        let t142 = (146.0 / 2025.0 * t133 - 73.0 / 540.0 * param_eta - 146.0 / 1215.0 + t138 / param_k1) * t24;
        let t143 = t142 * t28;
        let t144 = t30 * t32;
        let t145 = t144 * t37;
        let t148 = -0.162742215233874e0 + 0.162742215233874e0 * t80 + 0.678092563474475e-2 * t117 * t58 - 0.59353125082804e-1 * t121 + t126 * t54 * t57 * t120 / 24.0 + t143 * t145 / 288.0;
        let t149 = t71 * t71;
        let t150 = t148 * t149;
        let t151 = t78 * t78;
        let t152 = 1.0 / t151;
        let t153 = t149 * t149;
        let t154 = t151 * t151;
        let t155 = 1.0 / t154;
        let t157 = t153 * t155 + 1.0;
        let t158 = 1.0 / t157;
        let t159 = t152 * t158;
        let t160 = param_da4 * param_da4;
        let t161 = 1.0 / t160;
        let t163 = param_dp4 * param_dp4;
        let t164 = t163 * t163;
        let t165 = 1.0 / t164;
        let t166 = t38 * t165;
        let t170 = f64::exp(-t121 * t161 - t31 * t166 / 288.0);
        let t171 = t159 * t170;
        let t174 = t112 * t113 + 2.0 * t150 * t171 + t65 + 1.0;
        let t176 = f64::sqrt(3.0);
        let t177 = 1.0 / t26;
        let t178 = t24 * t177;
        let t179 = f64::sqrt(sigma[ip]);
        let t180 = t179 * t32;
        let t182 = 1.0 / t20 / rho[ip];
        let t184 = t178 * t180 * t182;
        let t185 = f64::sqrt(t184);
        let t189 = f64::exp(-0.98958e1 * t176 / t185);
        let t190 = 1.0 - t189;
        let t194 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t174 * t190);
        let tzk0 = 2.0 * t194;
        zk[ip] += tzk0;
    }
}
