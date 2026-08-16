//! MGGA_C_BC95 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_bc95.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_bc95_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_copp: f64,
    param_css: f64,
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
        let t3 = rho0 - rho1;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t6 = t3 * t5;
        let t7 = 1.0 + t6;
        let t8 = t7 <= zeta_threshold;
        let t9 = rho0 <= dens_threshold || t8;
        let t10 = piecewise3(t8, zeta_threshold, t7);
        let t11 = M_CBRT3;
        let t12 = 1.0 / M_PI;
        let t13 = pow_1_3(t12);
        let t14 = t11 * t13;
        let t15 = M_CBRT4;
        let t16 = t15 * t15;
        let t17 = t14 * t16;
        let t18 = pow_1_3(t4);
        let t19 = 1.0 / t18;
        let t20 = M_CBRT2;
        let t21 = t19 * t20;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = 1.0 / t22;
        let t24 = pow_1_3(t7);
        let t26 = piecewise3(t8, t23, 1.0 / t24);
        let t28 = t17 * t21 * t26;
        let t30 = 1.0 + 0.53425e-1 * t28;
        let t31 = f64::sqrt(t28);
        let t34 = pow_3_2(t28);
        let t36 = t11 * t11;
        let t37 = t13 * t13;
        let t38 = t36 * t37;
        let t39 = t38 * t15;
        let t40 = t18 * t18;
        let t41 = 1.0 / t40;
        let t42 = t20 * t20;
        let t43 = t41 * t42;
        let t44 = t26 * t26;
        let t46 = t39 * t43 * t44;
        let t48 = 0.379785e1 * t31 + 0.8969e0 * t28 + 0.204775e0 * t34 + 0.123235e0 * t46;
        let t51 = 1.0 + 0.16081979498692535067e2 / t48;
        let t52 = f64::ln(t51);
        let t54 = 0.621814e-1 * t30 * t52;
        let t56 = t22 * zeta_threshold;
        let t58 = piecewise3(2.0 <= zeta_threshold, t56, 2.0 * t20);
        let t60 = piecewise3(0.0 <= zeta_threshold, t56, 0.0);
        let t64 = 1.0 / (2.0 * t20 - 2.0);
        let t65 = (t58 + t60 - 2.0) * t64;
        let t67 = 1.0 + 0.5137e-1 * t28;
        let t72 = 0.705945e1 * t31 + 0.1549425e1 * t28 + 0.420775e0 * t34 + 0.1562925e0 * t46;
        let t75 = 1.0 + 0.32163958997385070134e2 / t72;
        let t76 = f64::ln(t75);
        let t80 = 1.0 + 0.278125e-1 * t28;
        let t85 = 0.51785e1 * t31 + 0.905775e0 * t28 + 0.1100325e0 * t34 + 0.1241775e0 * t46;
        let t88 = 1.0 + 0.29608749977793437516e2 / t85;
        let t89 = f64::ln(t88);
        let t90 = t80 * t89;
        let t96 = -t54 + t65 * (-0.310907e-1 * t67 * t76 + t54 - 0.19751673498613801407e-1 * t90) + 0.19751673498613801407e-1 * t65 * t90;
        let t99 = piecewise3(t9, 0.0, t10 * t96 / 2.0);
        let t100 = t99 * tau0;
        let t101 = pow_1_3(rho0);
        let t102 = t101 * t101;
        let t104 = 1.0 / t102 / rho0;
        let t108 = 1.0 / tau0;
        let t111 = 1.0 - sigma0 / rho0 * t108 / 8.0;
        let t112 = M_CBRT6;
        let t113 = t111 * t112;
        let t114 = M_PI * M_PI;
        let t115 = pow_1_3(t114);
        let t116 = t115 * t115;
        let t117 = 1.0 / t116;
        let t118 = param_css * sigma0;
        let t119 = rho0 * rho0;
        let t121 = 1.0 / t102 / t119;
        let t123 = t118 * t121 + 1.0;
        let t124 = t123 * t123;
        let t125 = 1.0 / t124;
        let t126 = t117 * t125;
        let t127 = t113 * t126;
        let t129 = 5.0 / 9.0 * t100 * t104 * t127;
        let t131 = 1.0 - t6;
        let t132 = t131 <= zeta_threshold;
        let t133 = rho1 <= dens_threshold || t132;
        let t134 = piecewise3(t132, zeta_threshold, t131);
        let t135 = pow_1_3(t131);
        let t137 = piecewise3(t132, t23, 1.0 / t135);
        let t139 = t17 * t21 * t137;
        let t141 = 1.0 + 0.53425e-1 * t139;
        let t142 = f64::sqrt(t139);
        let t145 = pow_3_2(t139);
        let t147 = t137 * t137;
        let t149 = t39 * t43 * t147;
        let t151 = 0.379785e1 * t142 + 0.8969e0 * t139 + 0.204775e0 * t145 + 0.123235e0 * t149;
        let t154 = 1.0 + 0.16081979498692535067e2 / t151;
        let t155 = f64::ln(t154);
        let t157 = 0.621814e-1 * t141 * t155;
        let t159 = 1.0 + 0.5137e-1 * t139;
        let t164 = 0.705945e1 * t142 + 0.1549425e1 * t139 + 0.420775e0 * t145 + 0.1562925e0 * t149;
        let t167 = 1.0 + 0.32163958997385070134e2 / t164;
        let t168 = f64::ln(t167);
        let t172 = 1.0 + 0.278125e-1 * t139;
        let t177 = 0.51785e1 * t142 + 0.905775e0 * t139 + 0.1100325e0 * t145 + 0.1241775e0 * t149;
        let t180 = 1.0 + 0.29608749977793437516e2 / t177;
        let t181 = f64::ln(t180);
        let t182 = t172 * t181;
        let t188 = -t157 + t65 * (-0.310907e-1 * t159 * t168 + t157 - 0.19751673498613801407e-1 * t182) + 0.19751673498613801407e-1 * t65 * t182;
        let t191 = piecewise3(t133, 0.0, t134 * t188 / 2.0);
        let t192 = t191 * tau1;
        let t193 = pow_1_3(rho1);
        let t194 = t193 * t193;
        let t196 = 1.0 / t194 / rho1;
        let t200 = 1.0 / tau1;
        let t203 = 1.0 - sigma2 / rho1 * t200 / 8.0;
        let t204 = t203 * t112;
        let t205 = param_css * sigma2;
        let t206 = rho1 * rho1;
        let t208 = 1.0 / t194 / t206;
        let t210 = t205 * t208 + 1.0;
        let t211 = t210 * t210;
        let t212 = 1.0 / t211;
        let t213 = t117 * t212;
        let t214 = t204 * t213;
        let t216 = 5.0 / 9.0 * t192 * t196 * t214;
        let t218 = t14 * t16 * t19;
        let t220 = 1.0 + 0.53425e-1 * t218;
        let t221 = f64::sqrt(t218);
        let t224 = pow_3_2(t218);
        let t227 = t38 * t15 * t41;
        let t229 = 0.379785e1 * t221 + 0.8969e0 * t218 + 0.204775e0 * t224 + 0.123235e0 * t227;
        let t232 = 1.0 + 0.16081979498692535067e2 / t229;
        let t233 = f64::ln(t232);
        let t235 = 0.621814e-1 * t220 * t233;
        let t236 = t3 * t3;
        let t237 = t236 * t236;
        let t238 = t4 * t4;
        let t239 = t238 * t238;
        let t240 = 1.0 / t239;
        let t241 = t237 * t240;
        let t242 = t24 * t7;
        let t243 = piecewise3(t8, t56, t242);
        let t244 = t135 * t131;
        let t245 = piecewise3(t132, t56, t244);
        let t246 = t243 + t245 - 2.0;
        let t247 = t246 * t64;
        let t249 = 1.0 + 0.5137e-1 * t218;
        let t254 = 0.705945e1 * t221 + 0.1549425e1 * t218 + 0.420775e0 * t224 + 0.1562925e0 * t227;
        let t257 = 1.0 + 0.32163958997385070134e2 / t254;
        let t258 = f64::ln(t257);
        let t262 = 1.0 + 0.278125e-1 * t218;
        let t267 = 0.51785e1 * t221 + 0.905775e0 * t218 + 0.1100325e0 * t224 + 0.1241775e0 * t227;
        let t270 = 1.0 + 0.29608749977793437516e2 / t267;
        let t271 = f64::ln(t270);
        let t272 = t262 * t271;
        let t274 = -0.310907e-1 * t249 * t258 + t235 - 0.19751673498613801407e-1 * t272;
        let t275 = t247 * t274;
        let t279 = -t235 + t241 * t275 + 0.19751673498613801407e-1 * t247 * t272 - t99 - t191;
        let t284 = 1.0 + param_copp * (sigma0 * t121 + sigma2 * t208);
        let t285 = 1.0 / t284;
        let t286 = t279 * t285;
        let tzk0 = t129 + t216 + t286;
        zk[ip] += tzk0;
    }
}
