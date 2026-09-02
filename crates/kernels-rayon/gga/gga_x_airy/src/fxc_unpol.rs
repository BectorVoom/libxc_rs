//! GGA_X_AIRY fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_airy_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = rmath::sqrt(sigma[ip]);
        let t27 = M_CBRT2;
        let t28 = t26 * t27;
        let t30 = 1.0 / t18 / rho[ip];
        let t32 = t25 * t28 * t30;
        let t33 = rmath::pow(t32, 2.626712);
        let t35 = 1.0 + 0.00013471619689594795 * t33;
        let t36 = rmath::pow(t35, -0.657946);
        let t39 = rmath::pow(t32, 3.217063);
        let t41 = rmath::pow(t32, 3.223476);
        let t43 = 1.0 - 0.04521241301076986 * t39 + 0.04540222195662038 * t41;
        let t44 = rmath::pow(t32, 3.473804);
        let t46 = 1.0 + 0.0004770218022490335 * t44;
        let t47 = 1.0 / t46;
        let t49 = 6.014601922021111e-05 * t33 * t36 + t43 * t47;
        let t53 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t49);
        let tzk0 = 2.0 * t53;
        zk[ip] += tzk0;
        let t54 = t18 * t18;
        let t56 = t17 / t54;
        let t60 = rmath::pow(t32, 1.626712);
        let t62 = t60 * t36 * t21;
        let t63 = t24 * t26;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t18 / t64;
        let t67 = t27 * t66;
        let t68 = t63 * t67;
        let t71 = rmath::pow(t32, 4.253424);
        let t72 = rmath::pow(t35, -1.657946);
        let t74 = t71 * t72 * t21;
        let t77 = rmath::pow(t32, 2.217063);
        let t79 = t77 * t21 * t24;
        let t80 = t28 * t66;
        let t83 = rmath::pow(t32, 2.223476);
        let t85 = t83 * t21 * t24;
        let t88 = 0.19393490805022173 * t79 * t80 - 0.19513729709845176 * t85 * t80;
        let t90 = t46 * t46;
        let t91 = 1.0 / t90;
        let t92 = t43 * t91;
        let t93 = rmath::pow(t32, 2.473804);
        let t94 = t93 * t21;
        let t95 = t92 * t94;
        let t98 = -0.00021064836058394556 * t62 * t68 + 1.8671024483029836e-08 * t74 * t68 + t88 * t47 + 0.0022094403263198687 * t95 * t68;
        let t103 = piecewise3(t2, 0.0, -t6 * t56 * t49 / 8.0 - 3.0 / 8.0 * t6 * t19 * t98);
        let tvrho0 = 2.0 * rho[ip] * t103 + 2.0 * t53;
        vrho[ip] += tvrho0;
        let t106 = 1.0 / t26;
        let t107 = t24 * t106;
        let t108 = t27 * t30;
        let t109 = t107 * t108;
        let t114 = t106 * t27;
        let t115 = t114 * t30;
        let t120 = -0.07272559051883315 * t79 * t115 + 0.07317648641191941 * t85 * t115;
        let t124 = 7.899313521897959e-05 * t62 * t109 - 7.001634181136188e-09 * t74 * t109 + t120 * t47 - 0.0008285401223699508 * t95 * t109;
        let t128 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t124);
        let tvsigma0 = 2.0 * rho[ip] * t128;
        vsigma[ip] += tvsigma0;
        let t133 = t17 / t54 / rho[ip];
        let t140 = rmath::pow(t32, 0.626712);
        let t142 = t140 * t36 * t20;
        let t143 = t23 * t23;
        let t144 = 1.0 / t143;
        let t145 = t144 * sigma[ip];
        let t146 = t27 * t27;
        let t147 = t64 * t64;
        let t149 = 1.0 / t54 / t147;
        let t150 = t146 * t149;
        let t151 = t145 * t150;
        let t154 = rmath::pow(t32, 3.253424);
        let t156 = t154 * t72 * t20;
        let t159 = t64 * rho[ip];
        let t161 = 1.0 / t18 / t159;
        let t162 = t27 * t161;
        let t163 = t63 * t162;
        let t166 = rmath::pow(t32, 5.880136);
        let t167 = rmath::pow(t35, -2.657946);
        let t169 = t166 * t167 * t20;
        let t174 = rmath::pow(t32, 1.217063);
        let t175 = t174 * t20;
        let t176 = t175 * t144;
        let t177 = sigma[ip] * t146;
        let t178 = t177 * t149;
        let t181 = t28 * t161;
        let t184 = rmath::pow(t32, 1.223476);
        let t185 = t184 * t20;
        let t186 = t185 * t144;
        let t191 = -3.4397272723723904 * t176 * t178 - 0.45251478545051743 * t79 * t181 + 3.471064774426217 * t186 * t178 + 0.45532035989638747 * t85 * t181;
        let t193 = t88 * t91;
        let t194 = t193 * t94;
        let t198 = 1.0 / t90 / t46;
        let t199 = t43 * t198;
        let t200 = rmath::pow(t32, 4.947608);
        let t201 = t200 * t20;
        let t202 = t199 * t201;
        let t205 = rmath::pow(t32, 1.473804);
        let t206 = t205 * t20;
        let t207 = t92 * t206;
        let t212 = 0.00274131372753785 * t142 * t151 - 1.0276735016205997e-06 * t156 * t151 + 0.0004915128413625396 * t62 * t163 + 8.763160960794521e-11 * t169 * t151 - 4.356572379373628e-08 * t74 * t163 + t191 * t47 + 0.004418880652639737 * t194 * t68 + 5.8579518666821375e-05 * t202 * t151 - 0.04372577853609117 * t207 * t151 - 0.005155360761413027 * t95 * t163;
        let t217 = piecewise3(t2, 0.0, t6 * t133 * t49 / 12.0 - t6 * t56 * t98 / 4.0 - 3.0 / 8.0 * t6 * t19 * t212);
        let tv2rho20 = 2.0 * rho[ip] * t217 + 4.0 * t103;
        v2rho2[ip] += tv2rho20;
        let t223 = t144 * t146;
        let t225 = 1.0 / t54 / t159;
        let t226 = t223 * t225;
        let t231 = t107 * t67;
        let t240 = t114 * t66;
        let t247 = 1.2898977271396463 * t175 * t226 + 0.09696745402511087 * t79 * t240 - 1.3016492904098316 * t185 * t226 - 0.09756864854922588 * t85 * t240;
        let t249 = t120 * t91;
        let t250 = t249 * t94;
        let t255 = t199 * t200;
        let t256 = t20 * t144;
        let t257 = t146 * t225;
        let t258 = t256 * t257;
        let t261 = t92 * t205;
        let t266 = -0.0010279926478266937 * t142 * t226 + 3.853775631077249e-07 * t156 * t226 - 0.00010532418029197278 * t62 * t231 - 3.2861853602979454e-11 * t169 * t226 + 9.335512241514918e-09 * t74 * t231 + t247 * t47 + 0.0022094403263198687 * t250 * t68 - 0.0008285401223699508 * t194 * t109 - 2.1967319500058017e-05 * t255 * t258 + 0.01639716695103419 * t261 * t258 + 0.0011047201631599344 * t95 * t231;
        let t271 = piecewise3(t2, 0.0, -t6 * t56 * t124 / 8.0 - 3.0 / 8.0 * t6 * t19 * t266);
        let tv2rhosigma0 = 2.0 * rho[ip] * t271 + 2.0 * t128;
        v2rhosigma[ip] += tv2rhosigma0;
        let t274 = 1.0 / sigma[ip];
        let t275 = t144 * t274;
        let t277 = 1.0 / t54 / t64;
        let t278 = t146 * t277;
        let t279 = t275 * t278;
        let t284 = t26 * sigma[ip];
        let t285 = 1.0 / t284;
        let t286 = t24 * t285;
        let t287 = t286 * t108;
        let t294 = t274 * t146;
        let t295 = t294 * t277;
        let t298 = t285 * t27;
        let t299 = t298 * t30;
        let t306 = -0.48371164767736735 * t176 * t295 + 0.036362795259416575 * t79 * t299 + 0.4881184839036868 * t186 * t295 - 0.03658824320595971 * t85 * t299;
        let t316 = 0.00038549724293501016 * t142 * t279 - 1.4451658616539682e-07 * t156 * t279 - 3.9496567609489795e-05 * t62 * t287 + 1.2323195101117295e-11 * t169 * t279 + 3.500817090568094e-09 * t74 * t287 + t306 * t47 - 0.0016570802447399015 * t250 * t109 + 8.237744812521756e-06 * t202 * t279 - 0.006148937606637821 * t207 * t279 + 0.0004142700611849754 * t95 * t287;
        let t320 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t316);
        let tv2sigma20 = 2.0 * rho[ip] * t320;
        v2sigma2[ip] += tv2sigma20;
    }
}
