//! MGGA_XC_CC06 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_xc_cc06.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_xc_cc06_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t9 = pow_1_3(zeta_threshold);
        let t11 = piecewise3(1.0 <= zeta_threshold, t9 * zeta_threshold, 1.0);
        let t12 = pow_1_3(rho[ip]);
        let t16 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t11 * t12);
        let t18 = 1.0 / M_PI;
        let t19 = pow_1_3(t18);
        let t20 = t4 * t19;
        let t21 = M_CBRT4;
        let t22 = t21 * t21;
        let t25 = t20 * t22 / t12;
        let t27 = 1.0 + 0.053425 * t25;
        let t28 = rmath::sqrt(t25);
        let t31 = pow_3_2(t25);
        let t33 = t4 * t4;
        let t34 = t19 * t19;
        let t35 = t33 * t34;
        let t36 = t12 * t12;
        let t37 = 1.0 / t36;
        let t39 = t35 * t21 * t37;
        let t41 = 3.79785 * t28 + 0.8969 * t25 + 0.204775 * t31 + 0.123235 * t39;
        let t44 = 1.0 + 16.081824322151103 / t41;
        let t45 = rmath::ln(t44);
        let t50 = M_CBRT2;
        let t54 = (2.0 * t11 - 2.0) / (2.0 * t50 - 2.0);
        let t56 = 1.0 + 0.0278125 * t25;
        let t61 = 5.1785 * t28 + 0.905775 * t25 + 0.1100325 * t31 + 0.1241775 * t39;
        let t64 = 1.0 + 29.608574643216677 / t61;
        let t65 = rmath::ln(t64);
        let t69 = 2.0 * t16 - 0.062182 * t27 * t45 + 0.019751789702565206 * t54 * t56 * t65;
        let t70 = t33 * t21;
        let t71 = t34 * lapl[ip];
        let t73 = 1.0 / t36 / rho[ip];
        let t75 = t70 * t71 * t73;
        let t77 = -0.0007 + 0.002 * t75;
        let t79 = 1.0 + 0.0065 * t75;
        let t80 = 1.0 / t79;
        let t82 = t77 * t80 + 1.0;
        let tzk0 = t69 * t82;
        zk[ip] += tzk0;
        let t86 = piecewise3(t3, 0.0, -t7 * t11 * t37 / 8.0);
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
        let t105 = rmath::sqrt(t25);
        let t106 = t105 * t4;
        let t107 = t106 * t100;
        let t109 = t21 * t73;
        let t110 = t35 * t109;
        let t112 = -0.632975 * t101 - 0.29896666666666666 * t103 - 0.1023875 * t107 - 0.08215666666666667 * t110;
        let t113 = 1.0 / t44;
        let t114 = t112 * t113;
        let t117 = t54 * t4;
        let t122 = t54 * t56;
        let t123 = t61 * t61;
        let t124 = 1.0 / t123;
        let t129 = -0.8630833333333333 * t101 - 0.301925 * t103 - 0.05501625 * t107 - 0.082785 * t110;
        let t131 = 1.0 / t64;
        let t132 = t124 * t129 * t131;
        let t135 = 2.0 * t86 + 0.0011073577833333333 * t20 * t90 * t45 + 1.0 * t96 * t114 - 0.0001831155503675316 * t117 * t99 * t89 * t65 - 0.5848223397455204 * t122 * t132;
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
        let t155 = -0.0033333333333333335 * t139 * t143 * t80 + 0.010833333333333334 * t150 * t151 * t143;
        let tvrho0 = t136 * t82 + t138 * t155 + tzk0;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t163 = 0.002 * t35 * t109 * t80 - 0.0065 * t150 * t151 * t73;
        let tvlapl0 = t138 * t163;
        vlapl[ip] += tvlapl0;
        let tvtau0 = 0.0;
        vtau[ip] += tvtau0;
        let t171 = piecewise3(t3, 0.0, t7 * t11 * t73 / 12.0);
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
        let t202 = 1.0/rmath::sqrt(t25);
        let t203 = t202 * t33;
        let t204 = t203 * t194;
        let t206 = t106 * t197;
        let t208 = t21 * t142;
        let t209 = t35 * t208;
        let t211 = -0.4219833333333333 * t195 + 0.8439666666666666 * t198 + 0.3986222222222222 * t200 + 0.06825833333333334 * t204 + 0.13651666666666668 * t206 + 0.1369277777777778 * t209;
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
        let t244 = -0.5753888888888888 * t195 + 1.1507777777777777 * t198 + 0.4025666666666667 * t200 + 0.0366775 * t204 + 0.073355 * t206 + 0.137975 * t209;
        let t246 = t124 * t244 * t131;
        let t249 = t123 * t123;
        let t250 = 1.0 / t249;
        let t251 = t250 * t233;
        let t252 = t64 * t64;
        let t253 = 1.0 / t252;
        let t254 = t251 * t253;
        let t257 = 2.0 * t171 - 0.0014764770444444443 * t20 * t175 * t45 - 0.035616666666666665 * t179 * t180 * t114 - 2.0 * t186 * t188 + 1.0 * t96 * t212 + 16.081824322151103 * t217 * t220 + 0.0002441540671567088 * t117 * t99 * t174 * t65 + 0.010843580882781523 * t227 * t90 * t132 + 1.169644679491041 * t122 * t235 - 0.5848223397455204 * t122 * t246 - 17.315755899375862 * t122 * t254;
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
        let t292 = 0.008888888888888889 * t139 * t265 * t80 - 0.00021666666666666666 * t271 * t277 * t148 + 0.0007041666666666666 * t284 * t285 * t277 - 0.028888888888888888 * t150 * t151 * t265;
        let tv2rho20 = 2.0 * t135 * t82 + 2.0 * t136 * t155 + t138 * t292 + 2.0 * t69 * t155 + t258 * t82;
        v2rho2[ip] += tv2rho20;
        let tv2rhosigma0 = 0.0;
        v2rhosigma[ip] += tv2rhosigma0;
        let t300 = 1.0 / t12 / t273;
        let t311 = -0.0033333333333333335 * t35 * t208 * t80 + 0.00013 * t271 * t300 * t148 * lapl[ip] - 0.0004225 * t284 * t285 * t300 * lapl[ip] + 0.010833333333333334 * t150 * t194;
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
        let t323 = -7.8e-05 * t313 * t316 * t148 + 0.0002535 * t284 * t285 * t315;
        let tv2lapl20 = t138 * t323;
        v2lapl2[ip] += tv2lapl20;
        let tv2lapltau0 = 0.0;
        v2lapltau[ip] += tv2lapltau0;
        let tv2tau20 = 0.0;
        v2tau2[ip] += tv2tau20;
    }
}
