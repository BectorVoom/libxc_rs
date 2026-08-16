//! MGGA_C_CC vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_cc_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t3 = sigma0 + 2.0 * sigma1 + sigma2;
        let t4 = rho0 + rho1;
        let t5 = t4 * t4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(t4);
        let t8 = t7 * t7;
        let t10 = 1.0 / t8 / t6;
        let t11 = t3 * t10;
        let t12 = pow_1_3(rho0);
        let t13 = t12 * t12;
        let t15 = 1.0 / t13 / rho0;
        let t16 = tau0 * t15;
        let t17 = rho0 - rho1;
        let t18 = 1.0 / t4;
        let t19 = t17 * t18;
        let t20 = 1.0 + t19;
        let t21 = t20 / 2.0;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = t23 * t21;
        let t26 = pow_1_3(rho1);
        let t27 = t26 * t26;
        let t29 = 1.0 / t27 / rho1;
        let t30 = tau1 * t29;
        let t31 = 1.0 - t19;
        let t32 = t31 / 2.0;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = t34 * t32;
        let t37 = t16 * t24 + t30 * t35;
        let t38 = 1.0 / t37;
        let t39 = t17 * t17;
        let t40 = t38 * t39;
        let t43 = 1.0 - t11 * t40 / 8.0;
        let t44 = M_CBRT3;
        let t45 = 1.0 / M_PI;
        let t46 = pow_1_3(t45);
        let t47 = t44 * t46;
        let t48 = M_CBRT4;
        let t49 = t48 * t48;
        let t52 = t47 * t49 / t7;
        let t54 = 1.0 + 0.53425e-1 * t52;
        let t55 = f64::sqrt(t52);
        let t58 = pow_3_2(t52);
        let t60 = t44 * t44;
        let t61 = t46 * t46;
        let t62 = t60 * t61;
        let t65 = t62 * t48 / t8;
        let t67 = 0.379785e1 * t55 + 0.8969e0 * t52 + 0.204775e0 * t58 + 0.123235e0 * t65;
        let t70 = 1.0 + 0.16081979498692535067e2 / t67;
        let t71 = f64::ln(t70);
        let t73 = 0.621814e-1 * t54 * t71;
        let t74 = t39 * t39;
        let t75 = 1.0 / t6;
        let t76 = t74 * t75;
        let t77 = t20 <= zeta_threshold;
        let t78 = pow_1_3(zeta_threshold);
        let t79 = t78 * zeta_threshold;
        let t80 = pow_1_3(t20);
        let t82 = piecewise3(t77, t79, t80 * t20);
        let t83 = t31 <= zeta_threshold;
        let t84 = pow_1_3(t31);
        let t86 = piecewise3(t83, t79, t84 * t31);
        let t87 = t82 + t86 - 2.0;
        let t88 = M_CBRT2;
        let t91 = 1.0 / (2.0 * t88 - 2.0);
        let t92 = t87 * t91;
        let t94 = 1.0 + 0.5137e-1 * t52;
        let t99 = 0.705945e1 * t55 + 0.1549425e1 * t52 + 0.420775e0 * t58 + 0.1562925e0 * t65;
        let t102 = 1.0 + 0.32163958997385070134e2 / t99;
        let t103 = f64::ln(t102);
        let t107 = 1.0 + 0.278125e-1 * t52;
        let t112 = 0.51785e1 * t55 + 0.905775e0 * t52 + 0.1100325e0 * t58 + 0.1241775e0 * t65;
        let t115 = 1.0 + 0.29608749977793437516e2 / t112;
        let t116 = f64::ln(t115);
        let t117 = t107 * t116;
        let t119 = -0.310907e-1 * t94 * t103 + t73 - 0.19751673498613801407e-1 * t117;
        let t120 = t92 * t119;
        let t124 = -t73 + t76 * t120 + 0.19751673498613801407e-1 * t92 * t117;
        let tzk0 = t43 * t124;
        zk[ip] += tzk0;
        let t125 = t6 * t4;
        let t127 = 1.0 / t8 / t125;
        let t128 = t3 * t127;
        let t130 = 7.0 / 12.0 * t128 * t40;
        let t131 = t37 * t37;
        let t132 = 1.0 / t131;
        let t133 = t132 * t39;
        let t134 = rho0 * rho0;
        let t136 = 1.0 / t13 / t134;
        let t137 = tau0 * t136;
        let t139 = 1.0 / t5;
        let t140 = t17 * t139;
        let t141 = t18 - t140;
        let t142 = t141 / 2.0;
        let t143 = t23 * t142;
        let t145 = -t142;
        let t146 = t34 * t145;
        let t149 = -5.0 / 3.0 * t137 * t24 + 5.0 / 3.0 * t16 * t143 + 5.0 / 3.0 * t30 * t146;
        let t150 = t133 * t149;
        let t153 = t38 * t17;
        let t155 = t11 * t153 / 4.0;
        let t156 = t130 + t11 * t150 / 8.0 - t155;
        let t157 = t4 * t156;
        let t159 = t4 * t43;
        let t161 = 1.0 / t7 / t4;
        let t162 = t49 * t161;
        let t165 = 0.11073470983333333333e-2 * t47 * t162 * t71;
        let t166 = t67 * t67;
        let t167 = 1.0 / t166;
        let t168 = t54 * t167;
        let t170 = 1.0 / t55 * t44;
        let t171 = t46 * t49;
        let t172 = t171 * t161;
        let t173 = t170 * t172;
        let t175 = t47 * t162;
        let t177 = f64::sqrt(t52);
        let t178 = t177 * t44;
        let t179 = t178 * t172;
        let t184 = t62 * t48 / t8 / t4;
        let t186 = -0.632975e0 * t173 - 0.29896666666666666667e0 * t175 - 0.1023875e0 * t179 - 0.82156666666666666667e-1 * t184;
        let t187 = 1.0 / t70;
        let t188 = t186 * t187;
        let t190 = 1.0 * t168 * t188;
        let t191 = t39 * t17;
        let t192 = t191 * t75;
        let t194 = 4.0 * t192 * t120;
        let t195 = 1.0 / t125;
        let t196 = t74 * t195;
        let t198 = 4.0 * t196 * t120;
        let t201 = piecewise3(t77, 0.0, 4.0 / 3.0 * t80 * t141);
        let t202 = -t141;
        let t205 = piecewise3(t83, 0.0, 4.0 / 3.0 * t84 * t202);
        let t207 = (t201 + t205) * t91;
        let t208 = t207 * t119;
        let t213 = t99 * t99;
        let t214 = 1.0 / t213;
        let t215 = t94 * t214;
        let t220 = -0.1176575e1 * t173 - 0.516475e0 * t175 - 0.2103875e0 * t179 - 0.104195e0 * t184;
        let t221 = 1.0 / t102;
        let t222 = t220 * t221;
        let t228 = t112 * t112;
        let t229 = 1.0 / t228;
        let t230 = t107 * t229;
        let t235 = -0.86308333333333333334e0 * t173 - 0.301925e0 * t175 - 0.5501625e-1 * t179 - 0.82785e-1 * t184;
        let t236 = 1.0 / t115;
        let t237 = t235 * t236;
        let t240 = 0.53237641966666666666e-3 * t47 * t162 * t103 + 1.0 * t215 * t222 - t165 - t190 + 0.18311447306006545054e-3 * t47 * t162 * t116 + 0.5848223622634646207e0 * t230 * t237;
        let t241 = t92 * t240;
        let t242 = t76 * t241;
        let t245 = t92 * t44;
        let t247 = t171 * t161 * t116;
        let t249 = 0.18311447306006545054e-3 * t245 * t247;
        let t250 = t92 * t107;
        let t252 = t229 * t235 * t236;
        let t254 = 0.5848223622634646207e0 * t250 * t252;
        let t255 = t165 + t190 + t194 - t198 + t76 * t208 + t242 + 0.19751673498613801407e-1 * t207 * t117 - t249 - t254;
        let tvrho0 = t157 * t124 + t159 * t255 + tzk0;
        vrho[ip * 2] += tvrho0;
        let t257 = -t18 - t140;
        let t258 = t257 / 2.0;
        let t259 = t23 * t258;
        let t261 = rho1 * rho1;
        let t263 = 1.0 / t27 / t261;
        let t264 = tau1 * t263;
        let t266 = -t258;
        let t267 = t34 * t266;
        let t270 = 5.0 / 3.0 * t16 * t259 - 5.0 / 3.0 * t264 * t35 + 5.0 / 3.0 * t30 * t267;
        let t271 = t133 * t270;
        let t274 = t130 + t11 * t271 / 8.0 + t155;
        let t275 = t4 * t274;
        let t279 = piecewise3(t77, 0.0, 4.0 / 3.0 * t80 * t257);
        let t280 = -t257;
        let t283 = piecewise3(t83, 0.0, 4.0 / 3.0 * t84 * t280);
        let t285 = (t279 + t283) * t91;
        let t286 = t285 * t119;
        let t290 = t165 + t190 - t194 - t198 + t76 * t286 + t242 + 0.19751673498613801407e-1 * t285 * t117 - t249 - t254;
        let tvrho1 = t275 * t124 + t159 * t290 + tzk0;
        vrho[ip * 2 + 1] += tvrho1;
        let t292 = t5 * t4;
        let t294 = 1.0 / t8 / t292;
        let t295 = t294 * t38;
        let t296 = t39 * t124;
        let t297 = t295 * t296;
        let tvsigma0 = -t297 / 8.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = -t297 / 4.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t300 = t294 * t3;
        let t301 = t300 * t132;
        let t302 = t39 * t15;
        let t303 = t24 * t124;
        let t304 = t302 * t303;
        let tvtau0 = t301 * t304 / 8.0;
        vtau[ip * 2] += tvtau0;
        let t306 = t39 * t29;
        let t307 = t35 * t124;
        let t308 = t306 * t307;
        let tvtau1 = t301 * t308 / 8.0;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
