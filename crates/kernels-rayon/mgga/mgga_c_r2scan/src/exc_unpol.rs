//! MGGA_C_R2SCAN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_r2scan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_r2scan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = pow_1_3(rho[ip]);
        let t10 = t7 / t8;
        let t11 = t5 * t10;
        let t13 = 1.0 + 0.053425 * t11;
        let t14 = rmath::sqrt(t11);
        let t16 = 0.8969 * t11;
        let t17 = pow_3_2(t11);
        let t18 = 0.204775 * t17;
        let t19 = t2 * t2;
        let t20 = t4 * t4;
        let t21 = t19 * t20;
        let t22 = t8 * t8;
        let t25 = t21 * t6 / t22;
        let t26 = 0.123235 * t25;
        let t27 = 3.79785 * t14 + t16 + t18 + t26;
        let t30 = 1.0 + 16.081979498692537 / t27;
        let t31 = rmath::ln(t30);
        let t33 = 0.0621814 * t13 * t31;
        let t34 = 1.0 <= zeta_threshold;
        let t35 = pow_1_3(zeta_threshold);
        let t37 = piecewise3(t34, t35 * zeta_threshold, 1.0);
        let t39 = 2.0 * t37 - 2.0;
        let t40 = M_CBRT2;
        let t41 = t40 - 1.0;
        let t43 = 1.0 / t41 / 2.0;
        let t44 = t39 * t43;
        let t46 = 1.0 + 0.0278125 * t11;
        let t48 = 0.905775 * t11;
        let t49 = 0.1100325 * t17;
        let t50 = 0.1241775 * t25;
        let t51 = 5.1785 * t14 + t48 + t49 + t50;
        let t54 = 1.0 + 29.608749977793437 / t51;
        let t55 = rmath::ln(t54);
        let t58 = 0.0197516734986138 * t44 * t46 * t55;
        let t59 = rmath::ln(2.0);
        let t60 = 1.0 - t59;
        let t61 = M_PI * M_PI;
        let t63 = t60 / t61;
        let t64 = t35 * t35;
        let t65 = piecewise3(t34, t64, 1.0);
        let t66 = t65 * t65;
        let t67 = t66 * t65;
        let t69 = 1.0 / t60;
        let t71 = 1.0 / t67;
        let t72 = t61 * t71;
        let t74 = rmath::exp(-(-t33 + t58) * t69 * t72);
        let t75 = t74 - 1.0;
        let t77 = 1.0 + 0.025 * t11;
        let t79 = 1.0 + 0.04445 * t11;
        let t80 = 1.0 / t79;
        let t81 = t77 * t80;
        let t82 = rho[ip] * rho[ip];
        let t84 = 1.0 / t8 / t82;
        let t88 = 1.0 / t66;
        let t90 = 1.0 / t4;
        let t93 = 1.0 / t75;
        let t94 = t6 * t69 * t93;
        let t95 = t88 * t19 * t90 * t94;
        let t99 = piecewise3(t34, t64 * zeta_threshold, 1.0);
        let t100 = 1.0 / t99;
        let t101 = t69 * t100;
        let t102 = t71 * t93;
        let t103 = rmath::sqrt(4.0);
        let t104 = t103 * t14;
        let t106 = 0.03138525 * t11;
        let t107 = 1.0 + 0.022225 * t104 + t106;
        let t108 = t107 * t107;
        let t113 = 1.0 - 2.363 * t41 * t39 * t43;
        let t114 = 1.0 / t108 * t113;
        let t115 = 1.0 / t14;
        let t116 = t103 * t115;
        let t118 = 0.04445 * t116 + 0.125541;
        let t122 = 1.898925 * t104 + t16 + t18 + t26;
        let t125 = 1.0 + 16.081979498692537 / t122;
        let t126 = rmath::ln(t125);
        let t128 = t122 * t122;
        let t129 = 1.0 / t128;
        let t130 = t13 * t129;
        let t132 = rmath::sqrt(t11);
        let t135 = 3.79785 * t116 + 3.5876 + 1.22865 * t132 + 0.24647 * t11;
        let t136 = 1.0 / t125;
        let t137 = t135 * t136;
        let t141 = 2.58925 * t104 + t48 + t49 + t50;
        let t144 = 1.0 + 29.608749977793437 / t141;
        let t145 = rmath::ln(t144);
        let t148 = t44 * t46;
        let t149 = t141 * t141;
        let t150 = 1.0 / t149;
        let t154 = 5.1785 * t116 + 3.6231 + 0.660195 * t132 + 0.248355 * t11;
        let t156 = 1.0 / t144;
        let t157 = t150 * t154 * t156;
        let t160 = 0.0285764 * t114 * t118 + 0.01328816518 * t126 - 1.0 * t130 * t137 - 0.0021973736767207856 * t44 * t145 + 0.5848223622634646 * t148 * t157;
        let t165 = 1.0 + 0.04445 * t14 + t106;
        let t166 = 1.0 / t165;
        let t172 = 5.0 * t5 * t10 * t160 - 45.0 * param_eta * (-0.0285764 * t166 * t113 + t33 - t58);
        let t174 = t101 * t102 * t172;
        let t175 = M_CBRT6;
        let t176 = pow_1_3(t61);
        let t177 = t176 * t176;
        let t178 = 1.0 / t177;
        let t179 = t175 * t178;
        let t180 = t40 * t40;
        let t181 = t179 * t180;
        let t183 = 1.0 / t22 / t82;
        let t184 = sigma[ip] * t183;
        let t185 = t175 * t175;
        let t187 = 1.0 / t176 / t61;
        let t188 = t185 * t187;
        let t189 = sigma[ip] * sigma[ip];
        let t190 = t40 * t189;
        let t191 = t82 * t82;
        let t192 = t191 * rho[ip];
        let t194 = 1.0 / t8 / t192;
        let t198 = rmath::exp(-0.2044460407889637 * t188 * t190 * t194);
        let t200 = t181 * t184 * t198;
        let t203 = 1.0 + 0.027439371595564633 * t81 * sigma[ip] * t84 * t40 * t95 + 0.043341108700271344 * t174 * t200;
        let t204 = pow_1_4(t203);
        let t206 = 1.0 - 1.0 / t204;
        let t208 = t206 * t75 + 1.0;
        let t209 = rmath::ln(t208);
        let t211 = t63 * t67 * t209;
        let t213 = 1.0 / t22 / rho[ip];
        let t216 = tau[ip] * t213 - t184 / 8.0;
        let t220 = param_eta * sigma[ip];
        let t223 = 3.0 / 20.0 * t185 * t177 * t40 + t220 * t183 / 8.0;
        let t224 = 1.0 / t223;
        let t225 = t216 * t224;
        let t226 = t225 <= 0.0;
        let t227 = 0.0 < t225;
        let t228 = piecewise3(t227, 0.0, t225);
        let t229 = 1.0 - t228;
        let t230 = 1.0 / t229;
        let t233 = rmath::exp(-0.64 * t228 * t230);
        let t234 = t225 <= 2.5;
        let t235 = 2.5 < t225;
        let t236 = piecewise3(t235, 2.5, t225);
        let t238 = t236 * t236;
        let t240 = t238 * t236;
        let t242 = t238 * t238;
        let t244 = t242 * t236;
        let t246 = t242 * t238;
        let t251 = piecewise3(t235, t225, 2.5);
        let t252 = 1.0 - t251;
        let t255 = rmath::exp(1.5 / t252);
        let t257 = piecewise5(t226, t233, t234, 1.0 - 0.64 * t236 - 0.4352 * t238 - 1.535685604549 * t240 + 3.061560252175 * t242 - 1.915710236206 * t244 + 0.516884468372 * t246 - 0.051848879792 * t242 * t240, -0.7 * t255);
        let t260 = rmath::exp(1.0 * t166);
        let t261 = t260 - 1.0;
        let t262 = t180 * sigma[ip];
        let t263 = t262 * t183;
        let t266 = 1.0 + 0.02133764210437636 * t179 * t263;
        let t267 = pow_1_4(t266);
        let t269 = 1.0 - 1.0 / t267;
        let t271 = t261 * t269 + 1.0;
        let t272 = rmath::ln(t271);
        let t276 = (-0.0285764 * t166 + 0.0285764 * t272) * t113 + t33 - t58 - t211;
        let t277 = t257 * t276;
        let tzk0 = -t33 + t58 + t211 + t277;
        zk[ip] += tzk0;
    }
}
