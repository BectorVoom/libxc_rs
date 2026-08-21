//! GGA_C_PBE vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_pbe_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.053425 * t10;
        let t13 = rmath::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 3.79785 * t13 + 0.8969 * t10 + 0.204775 * t16 + 0.123235 * t24;
        let t29 = 1.0 + 16.081979498692537 / t26;
        let t30 = rmath::ln(t29);
        let t32 = 0.0621814 * t12 * t30;
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.0278125 * t10;
        let t50 = 5.1785 * t13 + 0.905775 * t10 + 0.1100325 * t16 + 0.1241775 * t24;
        let t53 = 1.0 + 29.608749977793437 / t50;
        let t54 = rmath::ln(t53);
        let t57 = 0.0197516734986138 * t43 * t45 * t54;
        let t58 = t34 * t34;
        let t59 = piecewise3(t33, t58, 1.0);
        let t60 = t59 * t59;
        let t61 = t60 * t59;
        let t62 = param_gamma * t61;
        let t63 = rho[ip] * rho[ip];
        let t65 = 1.0 / t7 / t63;
        let t68 = 1.0 / t60;
        let t70 = 1.0 / t3;
        let t72 = t68 * t18 * t70 * t5;
        let t75 = param_BB * param_beta;
        let t76 = 1.0 / param_gamma;
        let t79 = 1.0 / t61;
        let t81 = rmath::exp(-(-t32 + t57) * t76 * t79);
        let t82 = t81 - 1.0;
        let t83 = 1.0 / t82;
        let t84 = t76 * t83;
        let t85 = sigma[ip] * sigma[ip];
        let t87 = t75 * t84 * t85;
        let t88 = t63 * t63;
        let t90 = 1.0 / t21 / t88;
        let t91 = t39 * t39;
        let t92 = t90 * t91;
        let t93 = t60 * t60;
        let t94 = 1.0 / t93;
        let t95 = t92 * t94;
        let t96 = 1.0 / t19;
        let t97 = t1 * t96;
        let t98 = t97 * t6;
        let t99 = t95 * t98;
        let t102 = sigma[ip] * t65 * t39 * t72 / 96.0 + t87 * t99 / 3072.0;
        let t103 = param_beta * t102;
        let t104 = param_beta * t76;
        let t107 = t104 * t83 * t102 + 1.0;
        let t108 = 1.0 / t107;
        let t109 = t76 * t108;
        let t111 = t103 * t109 + 1.0;
        let t112 = rmath::ln(t111);
        let t113 = t62 * t112;
        let tzk0 = -t32 + t57 + t113;
        zk[ip] += tzk0;
        let t115 = 1.0 / t7 / rho[ip];
        let t116 = t6 * t115;
        let t118 = t4 * t116 * t30;
        let t119 = 0.0011073470983333333 * t118;
        let t120 = t26 * t26;
        let t121 = 1.0 / t120;
        let t122 = t12 * t121;
        let t124 = 1.0 / t13 * t1;
        let t125 = t3 * t6;
        let t126 = t125 * t115;
        let t127 = t124 * t126;
        let t129 = t4 * t116;
        let t131 = rmath::sqrt(t10);
        let t132 = t131 * t1;
        let t133 = t132 * t126;
        let t138 = t20 * t5 / t21 / rho[ip];
        let t140 = -0.632975 * t127 - 0.29896666666666666 * t129 - 0.1023875 * t133 - 0.08215666666666667 * t138;
        let t141 = 1.0 / t29;
        let t142 = t140 * t141;
        let t143 = t122 * t142;
        let t144 = 1.0 * t143;
        let t145 = t43 * t1;
        let t148 = t145 * t125 * t115 * t54;
        let t149 = 0.00018311447306006544 * t148;
        let t150 = t43 * t45;
        let t151 = t50 * t50;
        let t152 = 1.0 / t151;
        let t157 = -0.8630833333333333 * t127 - 0.301925 * t129 - 0.05501625 * t133 - 0.082785 * t138;
        let t159 = 1.0 / t53;
        let t160 = t152 * t157 * t159;
        let t161 = t150 * t160;
        let t162 = 0.5848223622634646 * t161;
        let t163 = t63 * rho[ip];
        let t165 = 1.0 / t7 / t163;
        let t170 = param_gamma * param_gamma;
        let t171 = 1.0 / t170;
        let t172 = t75 * t171;
        let t173 = t82 * t82;
        let t174 = 1.0 / t173;
        let t175 = t174 * t85;
        let t176 = t175 * t90;
        let t177 = t172 * t176;
        let t179 = 1.0 / t93 / t61;
        let t180 = t91 * t179;
        let t181 = t180 * t1;
        let t182 = t96 * t6;
        let t183 = t119 + t144 - t149 - t162;
        let t184 = t183 * t81;
        let t185 = t182 * t184;
        let t186 = t181 * t185;
        let t189 = t88 * rho[ip];
        let t191 = 1.0 / t21 / t189;
        let t192 = t191 * t91;
        let t193 = t192 * t94;
        let t194 = t193 * t98;
        let t197 = -7.0 / 288.0 * sigma[ip] * t165 * t39 * t72 + t177 * t186 / 3072.0 - 7.0 / 4608.0 * t87 * t194;
        let t198 = param_beta * t197;
        let t200 = t107 * t107;
        let t201 = 1.0 / t200;
        let t202 = t76 * t201;
        let t204 = param_beta * t171 * t174;
        let t206 = t79 * t81;
        let t211 = t204 * t102 * t183 * t206 + t104 * t83 * t197;
        let t212 = t202 * t211;
        let t214 = -t103 * t212 + t198 * t109;
        let t215 = 1.0 / t111;
        let t217 = t62 * t214 * t215;
        let tvrho0 = -t32 + t57 + t113 + rho[ip] * (t119 + t144 - t149 - t162 + t217);
        vrho[ip] += tvrho0;
        let t220 = rho[ip] * param_gamma;
        let t224 = t18 * t70 * t5;
        let t228 = t75 * t84 * sigma[ip];
        let t231 = t65 * t39 * t68 * t224 / 96.0 + t228 * t99 / 1536.0;
        let t232 = param_beta * t231;
        let t234 = param_beta * param_beta;
        let t235 = t234 * t102;
        let t236 = t235 * t171;
        let t237 = t201 * t83;
        let t238 = t237 * t231;
        let t240 = t232 * t109 - t236 * t238;
        let tvsigma0 = t220 * t61 * t240 * t215;
        vsigma[ip] += tvsigma0;
    }
}
