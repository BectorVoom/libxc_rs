//! MGGA_XC_CC06 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_xc_cc06_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2rholapl: &mut Array<f64>,
    v2rhotau: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    v2sigmalapl: &mut Array<f64>,
    v2sigmatau: &mut Array<f64>,
    v2lapl2: &mut Array<f64>,
    v2lapltau: &mut Array<f64>,
    v2tau2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t9 = pow_1_3::<f64>(zeta_threshold);
        let t11 = piecewise3::<f64>(1.0 <= zeta_threshold, t9 * zeta_threshold, 1.0);
        let t12 = pow_1_3::<f64>(rho[ip]);
        let t16 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t7 * t11 * t12);
        let t18 = 1.0 / M_PI;
        let t19 = pow_1_3::<f64>(t18);
        let t20 = t4 * t19;
        let t21 = M_CBRT4;
        let t22 = t21 * t21;
        let t25 = t20 * t22 / t12;
        let t27 = 1.0 + 0.53425e-1 * t25;
        let t28 = f64::sqrt(t25);
        let t31 = pow_3_2::<f64>(t25);
        let t33 = t4 * t4;
        let t34 = t19 * t19;
        let t35 = t33 * t34;
        let t36 = t12 * t12;
        let t37 = 1.0 / t36;
        let t39 = t35 * t21 * t37;
        let t41 = 0.379785e1 * t28 + 0.8969e0 * t25 + 0.204775e0 * t31 + 0.123235e0 * t39;
        let t44 = 1.0 + 0.16081824322151104822e2 / t41;
        let t45 = f64::ln(t44);
        let t50 = M_CBRT2;
        let t54 = (2.0 * t11 - 2.0) / (2.0 * t50 - 2.0);
        let t56 = 1.0 + 0.278125e-1 * t25;
        let t61 = 0.51785e1 * t28 + 0.905775e0 * t25 + 0.1100325e0 * t31 + 0.1241775e0 * t39;
        let t64 = 1.0 + 0.29608574643216675549e2 / t61;
        let t65 = f64::ln(t64);
        let t69 = 2.0 * t16 - 0.62182e-1 * t27 * t45 + 0.19751789702565206229e-1 * t54 * t56 * t65;
        let t70 = t33 * t21;
        let t71 = t34 * lapl[ip];
        let t73 = 1.0 / t36 / rho[ip];
        let t75 = t70 * t71 * t73;
        let t77 = -0.7e-3 + 0.2e-2 * t75;
        let t79 = 1.0 + 0.65e-2 * t75;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let tzk0 = t69 * t82;
        zk[ip] += tzk0;
        let t86 = piecewise3::<f64>(t3, 0.0, -t7 * t11 * t37 / 8.0);
        let t89 = 1.0 / t12 / rho[ip];
        let t90 = t22 * t89;
        let t94 = t41 * t41;
        let t95 = 1.0 / t94;
        let t96 = t27 * t95;
        let t98 = 1.0 / t28 * t4;
        let t99 = t19 * t22;
        let t100 = t99 * t89;
        let t101 = t98 * t100;
        let t103 = t20 * t90;
        let t105 = f64::sqrt(t25);
        let t106 = t105 * t4;
        let t107 = t106 * t100;
        let t109 = t21 * t73;
        let t110 = t35 * t109;
        let t112 = -0.632975e0 * t101 - 0.29896666666666666667e0 * t103 - 0.1023875e0 * t107 - 0.82156666666666666667e-1 * t110;
        let t113 = 1.0 / t44;
        let t114 = t112 * t113;
        let t117 = t54 * t4;
        let t122 = t54 * t56;
        let t123 = t61 * t61;
        let t124 = 1.0 / t123;
        let t129 = -0.86308333333333333334e0 * t101 - 0.301925e0 * t103 - 0.5501625e-1 * t107 - 0.82785e-1 * t110;
        let t131 = 1.0 / t64;
        let t132 = t124 * t129 * t131;
        let t135 = 2.0 * t86 + 0.11073577833333333333e-2 * t20 * t90 * t45 + 1.0 * t96 * t114 - 0.18311555036753159941e-3 * t117 * t99 * t89 * t65 - 0.58482233974552040708e0 * t122 * t132;
        let t136 = rho[ip] * t135;
        let t138 = rho[ip] * t69;
        let t139 = t70 * t34;
        let t140 = rho[ip] * rho[ip];
        let t142 = 1.0 / t36 / t140;
        let t143 = lapl[ip] * t142;
        let t147 = t79 * t79;
        let t148 = 1.0 / t147;
        let t150 = t77 * t148 * t33;
        let t151 = t21 * t34;
        let t155 = -0.33333333333333333333e-2 * t139 * t143 * t80 + 0.10833333333333333333e-1 * t150 * t151 * t143;
        let tvrho0 = t136 * t82 + t138 * t155 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t163 = 0.2e-2 * t35 * t109 * t80 - 0.65e-2 * t150 * t151 * t73;
        let tvlapl0 = t138 * t163;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t171 = piecewise3::<f64>(t3, 0.0, t7 * t11 * t73 / 12.0);
        let t174 = 1.0 / t12 / t140;
        let t175 = t22 * t174;
        let t179 = t20 * t22;
        let t180 = t89 * t95;
        let t184 = t94 * t41;
        let t185 = 1.0 / t184;
        let t186 = t27 * t185;
        let t187 = t112 * t112;
        let t188 = t187 * t113;
        let t193 = 1.0 / t28 / t25 * t33;
        let t194 = t151 * t142;
        let t195 = t193 * t194;
        let t197 = t99 * t174;
        let t198 = t98 * t197;
        let t200 = t20 * t175;
        let t202 = 1.0/f64::sqrt(t25);
        let t203 = t202 * t33;
        let t204 = t203 * t194;
        let t206 = t106 * t197;
        let t208 = t21 * t142;
        let t209 = t35 * t208;
        let t211 = -0.42198333333333333333e0 * t195 + 0.84396666666666666666e0 * t198 + 0.39862222222222222223e0 * t200 + 0.68258333333333333333e-1 * t204 + 0.13651666666666666667e0 * t206 + 0.13692777777777777778e0 * t209;
        let t212 = t211 * t113;
        let t215 = t94 * t94;
        let t216 = 1.0 / t215;
        let t217 = t27 * t216;
        let t218 = t44 * t44;
        let t219 = 1.0 / t218;
        let t220 = t187 * t219;
        let t227 = t54 * t20;
        let t231 = t123 * t61;
        let t232 = 1.0 / t231;
        let t233 = t129 * t129;
        let t235 = t232 * t233 * t131;
        let t244 = -0.57538888888888888889e0 * t195 + 0.11507777777777777778e1 * t198 + 0.40256666666666666667e0 * t200 + 0.366775e-1 * t204 + 0.73355e-1 * t206 + 0.137975e0 * t209;
        let t246 = t124 * t244 * t131;
        let t249 = t123 * t123;
        let t250 = 1.0 / t249;
        let t251 = t250 * t233;
        let t252 = t64 * t64;
        let t253 = 1.0 / t252;
        let t254 = t251 * t253;
        let t257 = 2.0 * t171 - 0.14764770444444444444e-2 * t20 * t175 * t45 - 0.35616666666666666667e-1 * t179 * t180 * t114 - 2.0 * t186 * t188 + 1.0 * t96 * t212 + 0.16081824322151104822e2 * t217 * t220 + 0.24415406715670879921e-3 * t117 * t99 * t174 * t65 + 0.10843580882781524214e-1 * t227 * t90 * t132 + 0.11696446794910408142e1 * t122 * t235 - 0.58482233974552040708e0 * t122 * t246 - 0.17315755899375863299e2 * t122 * t254;
        let t258 = rho[ip] * t257;
        let t262 = t140 * rho[ip];
        let t264 = 1.0 / t36 / t262;
        let t265 = lapl[ip] * t264;
        let t270 = t19 * t18;
        let t271 = t4 * t22 * t270;
        let t272 = lapl[ip] * lapl[ip];
        let t273 = t140 * t140;
        let t274 = t273 * rho[ip];
        let t276 = 1.0 / t12 / t274;
        let t277 = t272 * t276;
        let t282 = 1.0 / t147 / t79;
        let t284 = t77 * t282 * t4;
        let t285 = t22 * t270;
        let t292 = 0.88888888888888888888e-2 * t139 * t265 * t80 - 0.21666666666666666666e-3 * t271 * t277 * t148 + 0.70416666666666666662e-3 * t284 * t285 * t277 - 0.28888888888888888888e-1 * t150 * t151 * t265;
        let tv2rho20 = 2.0 * t135 * t82 + 2.0 * t136 * t155 + t138 * t292 + 2.0 * t69 * t155 + t258 * t82;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = 1.0 / t12 / t273;
        let t311 = -0.33333333333333333333e-2 * t35 * t208 * t80 + 0.13e-3 * t271 * t300 * t148 * lapl[ip] - 0.42249999999999999999e-3 * t284 * t285 * t300 * lapl[ip] + 0.10833333333333333333e-1 * t150 * t194;
        let tv2rholapl0 = t136 * t163 + t138 * t311 + t69 * t163;
        v2rholapl[ip] += tv2rholapl0;
        let tv2rhotau0 = 0.0;
        v2rhotau[ip] += tv2rhotau0;
        let tv2sigma20 = 0.0;
        v2sigma2[ip] += tv2sigma20;
        let tv2sigmalapl0 = 0.0;
        v2sigmalapl[ip] += tv2sigmalapl0;
        let tv2sigmatau0 = 0.0;
        v2sigmatau[ip] += tv2sigmatau0;
        let t313 = t4 * t270;
        let t315 = 1.0 / t12 / t262;
        let t316 = t22 * t315;
        let t323 = -0.78e-4 * t313 * t316 * t148 + 0.2535e-3 * t284 * t285 * t315;
        let tv2lapl20 = t138 * t323;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
