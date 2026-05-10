//! GGA_C_P86VWN vxc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 76 shared lines across all orders.
//! Delta: 82 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_c_p86vwn_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mdelta: f64,
    param_mgamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (76 lines) ---
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = f64::sqrt(t10);
        let t14 = t11 + 0.186372e1 * t12 + 0.129352e2;
        let t15 = 1.0 / t14;
        let t19 = f64::ln(t4 * t9 * t15 / 4.0);
        let t20 = 0.310907e-1 * t19;
        let t21 = t12 + 0.372744e1;
        let t24 = f64::atan(0.61519908197590802322e1 / t21);
        let t25 = 0.38783294878113014393e-1 * t24;
        let t26 = t12 / 2.0;
        let t27 = t26 + 0.10498e0;
        let t28 = t27 * t27;
        let t30 = f64::ln(t28 * t15);
        let t31 = 0.96902277115443742139e-3 * t30;
        let t32 = M_PI * M_PI;
        let t33 = 1.0 / t32;
        let t35 = t11 + 0.565535e0 * t12 + 0.130045e2;
        let t36 = 1.0 / t35;
        let t40 = f64::ln(t4 * t9 * t36 / 4.0);
        let t41 = t12 + 0.113107e1;
        let t44 = f64::atan(0.71231089178181179908e1 / t41);
        let t46 = t26 + 0.47584e-2;
        let t47 = t46 * t46;
        let t49 = f64::ln(t47 * t36);
        let t53 = 1.0 <= zeta_threshold;
        let t54 = pow_1_3(zeta_threshold);
        let t56 = piecewise3(t53, t54 * zeta_threshold, 1.0);
        let t59 = M_CBRT2;
        let t60 = t59 - 1.0;
        let t65 = 9.0 * t56 - 9.0;
        let t67 = t33 * (t40 + 0.317708004743941464e0 * t44 + 0.41403379428206274608e-3 * t49) * t65 / 24.0;
        let t68 = rho[ip] * rho[ip];
        let t70 = 1.0 / t7 / t68;
        let t71 = sigma[ip] * t70;
        let t72 = param_aa + param_bb;
        let t73 = param_ftilde * t72;
        let t74 = param_malpha * t1;
        let t75 = t3 * t6;
        let t76 = t75 * t8;
        let t79 = t1 * t1;
        let t80 = param_mbeta * t79;
        let t81 = t3 * t3;
        let t82 = t81 * t5;
        let t83 = t7 * t7;
        let t84 = 1.0 / t83;
        let t85 = t82 * t84;
        let t88 = param_bb + t74 * t76 / 4.0 + t80 * t85 / 4.0;
        let t89 = param_mgamma * t1;
        let t92 = param_mdelta * t79;
        let t95 = 1.0 / rho[ip];
        let t98 = 1.0 + t89 * t76 / 4.0 + t92 * t85 / 4.0 + 0.23873241463784300365e4 * param_mbeta * t95;
        let t99 = 1.0 / t98;
        let t101 = t88 * t99 + param_aa;
        let t102 = 1.0 / t101;
        let t103 = f64::sqrt(sigma[ip]);
        let t104 = t102 * t103;
        let t105 = f64::powf(rho[ip], 1.0 / 6.0);
        let t107 = 1.0 / t105 / rho[ip];
        let t110 = f64::exp(-t73 * t104 * t107);
        let t112 = t54 * t54;
        let t114 = piecewise3(t53, t112 * zeta_threshold, 1.0);
        let t115 = f64::sqrt(t114);
        let t116 = 1.0 / t115;
        let t117 = t110 * t101 * t116;
        let t118 = t71 * t117;
        let tzk0 = t20 + t25 + t31 - t67 + t118;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (82 lines) ---
        let t120 = 1.0 / t7 / rho[ip];
        let t121 = t6 * t120;
        let t125 = t4 * t6;
        let t126 = t14 * t14;
        let t127 = 1.0 / t126;
        let t128 = t8 * t127;
        let t129 = t4 * t121;
        let t130 = t129 / 12.0;
        let t131 = 1.0 / t12;
        let t132 = t131 * t1;
        let t133 = t75 * t120;
        let t134 = t132 * t133;
        let t136 = -t130 - 0.31062e0 * t134;
        let t142 = 1.0 / t3;
        let t143 = (-t4 * t121 * t15 / 12.0 - t125 * t128 * t136 / 4.0) * t79 * t142;
        let t144 = t5 * t7;
        let t145 = t144 * t14;
        let t146 = t143 * t145;
        let t148 = t21 * t21;
        let t149 = 1.0 / t148;
        let t151 = t149 * t131 * t1;
        let t153 = 0.378469910464e2 * t149 + 1.0;
        let t154 = 1.0 / t153;
        let t157 = t151 * t75 * t120 * t154;
        let t159 = t27 * t15;
        let t160 = t159 * t131;
        let t163 = t28 * t127;
        let t165 = -t160 * t129 / 6.0 - t163 * t136;
        let t166 = 1.0 / t28;
        let t167 = t165 * t166;
        let t168 = t167 * t14;
        let t173 = t35 * t35;
        let t174 = 1.0 / t173;
        let t175 = t8 * t174;
        let t177 = -t130 - 0.94255833333333333334e-1 * t134;
        let t183 = (-t4 * t121 * t36 / 12.0 - t125 * t175 * t177 / 4.0) * t79 * t142;
        let t184 = t144 * t35;
        let t187 = t41 * t41;
        let t188 = 1.0 / t187;
        let t190 = t188 * t131 * t1;
        let t192 = 0.50738680655100000001e2 * t188 + 1.0;
        let t193 = 1.0 / t192;
        let t198 = t46 * t36;
        let t199 = t198 * t131;
        let t202 = t47 * t174;
        let t204 = -t199 * t129 / 6.0 - t202 * t177;
        let t205 = 1.0 / t47;
        let t206 = t204 * t205;
        let t211 = t33 * (t183 * t184 / 3.0 + 0.37717812030896172975e0 * t190 * t75 * t120 * t193 + 0.41403379428206274608e-3 * t206 * t35) * t65;
        let t213 = t68 * rho[ip];
        let t215 = 1.0 / t7 / t213;
        let t216 = sigma[ip] * t215;
        let t217 = t216 * t117;
        let t219 = t101 * t101;
        let t220 = 1.0 / t219;
        let t221 = t73 * t220;
        let t222 = t103 * t107;
        let t226 = 1.0 / t83 / rho[ip];
        let t227 = t82 * t226;
        let t230 = -t74 * t133 / 12.0 - t80 * t227 / 6.0;
        let t232 = t98 * t98;
        let t233 = 1.0 / t232;
        let t234 = t88 * t233;
        let t242 = -t89 * t133 / 12.0 - t92 * t227 / 6.0 - 0.23873241463784300365e4 * param_mbeta / t68;
        let t244 = t230 * t99 - t234 * t242;
        let t248 = 1.0 / t105 / t68;
        let t252 = t221 * t222 * t244 + 7.0 / 6.0 * t73 * t104 * t248;
        let t253 = t71 * t252;
        let t254 = t253 * t117;
        let t256 = t110 * t244 * t116;
        let t257 = t71 * t256;
        let tvrho0 = t20 + t25 + t31 - t67 + t118 + rho[ip] * (0.10363566666666666667e-1 * t146 + 0.39765745675026770179e-1 * t157 + 0.96902277115443742139e-3 * t168 - t211 / 24.0 - 7.0 / 3.0 * t217 + t254 + t257);
        vrho[ip] += tvrho0;
        let t260 = t70 * t110;
        let t261 = t101 * t116;
        let t262 = t260 * t261;
        let t263 = f64::sqrt(rho[ip]);
        let t265 = 1.0 / t263 / t213;
        let t266 = t103 * t265;
        let t267 = t266 * param_ftilde;
        let t269 = t72 * t110 * t116;
        let t271 = t267 * t269 / 2.0;
        let tvsigma0 = rho[ip] * (t262 - t271);
        vsigma[ip] += tvsigma0;
    }
}
