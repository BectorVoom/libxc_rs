//! GGA_XC_B97 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_b97.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_b97_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_c_x_1: f64,
    param_c_x_2: f64,
    param_c_x_3: f64,
    param_c_x_4: f64,
    param_c_x_0: f64,
    param_c_ss_1: f64,
    param_c_ss_2: f64,
    param_c_ss_3: f64,
    param_c_ss_4: f64,
    param_c_ss_0: f64,
    param_c_ab_1: f64,
    param_c_ab_2: f64,
    param_c_ab_3: f64,
    param_c_ab_4: f64,
    param_c_ab_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = piecewise3(t3, zeta_threshold, 1.0);
        let t6 = pow_1_3(zeta_threshold);
        let t8 = piecewise3(t3, 1.0 / t6, 1.0);
        let t9 = t8 * t8;
        let t10 = t9 * t8;
        let t14 = rho[ip] / t10 / 2.0 <= dens_threshold;
        let t15 = M_CBRT3;
        let t16 = M_CBRTPI;
        let t19 = M_CBRT2;
        let t20 = t19 * t19;
        let t21 = t15 / t16 * t20;
        let t23 = t6 * zeta_threshold;
        let t25 = piecewise3(2.0 <= zeta_threshold, t23, 2.0 * t19);
        let t26 = pow_1_3(rho[ip]);
        let t28 = 1.0 / t8;
        let t32 = piecewise3(t14, 0.0, -3.0 / 16.0 * t21 * t25 * t26 * t28);
        let t33 = 0.0 <= dens_threshold;
        let t35 = piecewise3(0.0 <= zeta_threshold, t23, 0.0);
        let t40 = piecewise3(t33, 0.0, -3.0 / 16.0 * t21 * t35 * t26 * t28);
        let t44 = piecewise3(t4, 0.0, t5 * (t32 + t40) / 2.0);
        let t46 = param_c_x_1;
        let t47 = t46 * sigma[ip];
        let t48 = rho[ip] * rho[ip];
        let t49 = t26 * t26;
        let t51 = 1.0 / t49 / t48;
        let t52 = t20 * t51;
        let t54 = sigma[ip] * t20 * t51;
        let t56 = 1.0 + 0.004 * t54;
        let t57 = 1.0 / t56;
        let t61 = param_c_x_2;
        let t62 = sigma[ip] * sigma[ip];
        let t63 = t61 * t62;
        let t64 = t48 * t48;
        let t65 = t64 * rho[ip];
        let t67 = 1.0 / t26 / t65;
        let t68 = t19 * t67;
        let t69 = t56 * t56;
        let t70 = 1.0 / t69;
        let t71 = t68 * t70;
        let t74 = param_c_x_3;
        let t75 = t62 * sigma[ip];
        let t76 = t74 * t75;
        let t77 = t64 * t64;
        let t78 = 1.0 / t77;
        let t79 = t69 * t56;
        let t80 = 1.0 / t79;
        let t81 = t78 * t80;
        let t84 = param_c_x_4;
        let t85 = t62 * t62;
        let t86 = t84 * t85;
        let t87 = t77 * t48;
        let t89 = 1.0 / t49 / t87;
        let t90 = t20 * t89;
        let t91 = t69 * t69;
        let t92 = 1.0 / t91;
        let t93 = t90 * t92;
        let t96 = param_c_x_0 + 0.004 * t47 * t52 * t57 + 3.2e-05 * t63 * t71 + 2.56e-07 * t76 * t81 + 1.024e-09 * t86 * t93;
        let t98 = 2.0 * t44 * t96;
        let t99 = 1.0 / M_PI;
        let t100 = pow_1_3(t99);
        let t101 = t15 * t100;
        let t102 = M_CBRT4;
        let t103 = t102 * t102;
        let t104 = t101 * t103;
        let t105 = 1.0 / t26;
        let t108 = t104 * t105 * t19 * t8;
        let t110 = 1.0 + 0.053425 * t108;
        let t111 = rmath::sqrt(t108);
        let t114 = pow_3_2(t108);
        let t116 = t15 * t15;
        let t117 = t100 * t100;
        let t118 = t116 * t117;
        let t119 = t118 * t102;
        let t120 = 1.0 / t49;
        let t123 = t119 * t120 * t20 * t9;
        let t125 = 3.79785 * t111 + 0.8969 * t108 + 0.204775 * t114 + 0.123235 * t123;
        let t128 = 1.0 + 16.081824322151103 / t125;
        let t129 = rmath::ln(t128);
        let t131 = 0.062182 * t110 * t129;
        let t135 = 1.0 / (2.0 * t19 - 2.0);
        let t136 = (t25 + t35 - 2.0) * t135;
        let t138 = 1.0 + 0.05137 * t108;
        let t143 = 7.05945 * t111 + 1.549425 * t108 + 0.420775 * t114 + 0.1562925 * t123;
        let t146 = 1.0 + 32.1646831778707 / t143;
        let t147 = rmath::ln(t146);
        let t151 = 1.0 + 0.0278125 * t108;
        let t156 = 5.1785 * t111 + 0.905775 * t108 + 0.1100325 * t114 + 0.1241775 * t123;
        let t159 = 1.0 + 29.608574643216677 / t156;
        let t160 = rmath::ln(t159);
        let t161 = t151 * t160;
        let t170 = piecewise3(t4, 0.0, t5 * (-t131 + t136 * (-0.03109 * t138 * t147 + t131 - 0.019751789702565206 * t161) + 0.019751789702565206 * t136 * t161) / 2.0);
        let t172 = param_c_ss_1;
        let t173 = t172 * sigma[ip];
        let t175 = 1.0 + 0.2 * t54;
        let t176 = 1.0 / t175;
        let t180 = param_c_ss_2;
        let t181 = t180 * t62;
        let t182 = t175 * t175;
        let t183 = 1.0 / t182;
        let t184 = t68 * t183;
        let t187 = param_c_ss_3;
        let t188 = t187 * t75;
        let t189 = t182 * t175;
        let t190 = 1.0 / t189;
        let t191 = t78 * t190;
        let t194 = param_c_ss_4;
        let t195 = t194 * t85;
        let t196 = t182 * t182;
        let t197 = 1.0 / t196;
        let t198 = t90 * t197;
        let t201 = param_c_ss_0 + 0.2 * t173 * t52 * t176 + 0.08 * t181 * t184 + 0.032 * t188 * t191 + 0.0064 * t195 * t198;
        let t203 = 2.0 * t170 * t201;
        let t205 = t101 * t103 * t105;
        let t207 = 1.0 + 0.053425 * t205;
        let t208 = rmath::sqrt(t205);
        let t211 = pow_3_2(t205);
        let t214 = t118 * t102 * t120;
        let t216 = 3.79785 * t208 + 0.8969 * t205 + 0.204775 * t211 + 0.123235 * t214;
        let t219 = 1.0 + 16.081824322151103 / t216;
        let t220 = rmath::ln(t219);
        let t223 = piecewise3(t3, t23, 1.0);
        let t226 = (2.0 * t223 - 2.0) * t135;
        let t228 = 1.0 + 0.0278125 * t205;
        let t233 = 5.1785 * t208 + 0.905775 * t205 + 0.1100325 * t211 + 0.1241775 * t214;
        let t236 = 1.0 + 29.608574643216677 / t233;
        let t237 = rmath::ln(t236);
        let t242 = -0.062182 * t207 * t220 + 0.019751789702565206 * t226 * t228 * t237 - 2.0 * t170;
        let t244 = param_c_ab_1;
        let t245 = t244 * sigma[ip];
        let t247 = 1.0 + 0.006 * t54;
        let t248 = 1.0 / t247;
        let t252 = param_c_ab_2;
        let t253 = t252 * t62;
        let t254 = t247 * t247;
        let t255 = 1.0 / t254;
        let t256 = t68 * t255;
        let t259 = param_c_ab_3;
        let t260 = t259 * t75;
        let t261 = t254 * t247;
        let t262 = 1.0 / t261;
        let t263 = t78 * t262;
        let t266 = param_c_ab_4;
        let t267 = t266 * t85;
        let t268 = t254 * t254;
        let t269 = 1.0 / t268;
        let t270 = t90 * t269;
        let t273 = param_c_ab_0 + 0.006 * t245 * t52 * t248 + 7.2e-05 * t253 * t256 + 8.64e-07 * t260 * t263 + 5.184e-09 * t267 * t270;
        let t274 = t242 * t273;
        let tzk0 = t98 + t203 + t274;
        zk[ip] += tzk0;
    }
}
