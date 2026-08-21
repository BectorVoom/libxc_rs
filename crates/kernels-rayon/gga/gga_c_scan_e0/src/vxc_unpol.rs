//! GGA_C_SCAN_E0 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_scan_e0.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_scan_e0_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let t58 = rmath::ln(2.0);
        let t59 = 1.0 - t58;
        let t60 = M_PI * M_PI;
        let t62 = t59 / t60;
        let t63 = t34 * t34;
        let t64 = piecewise3(t33, t63, 1.0);
        let t65 = t64 * t64;
        let t66 = t65 * t64;
        let t68 = 1.0 + 0.025 * t10;
        let t70 = 1.0 + 0.04445 * t10;
        let t71 = 1.0 / t70;
        let t72 = t68 * t71;
        let t73 = 1.0 / t59;
        let t76 = 1.0 / t66;
        let t77 = t60 * t76;
        let t79 = rmath::exp(-(-t32 + t57) * t73 * t77);
        let t80 = t79 - 1.0;
        let t81 = 1.0 / t80;
        let t82 = t73 * t81;
        let t83 = t82 * sigma[ip];
        let t84 = t72 * t83;
        let t85 = rho[ip] * rho[ip];
        let t87 = 1.0 / t7 / t85;
        let t88 = t87 * t39;
        let t89 = 1.0 / t65;
        let t91 = 1.0 / t3;
        let t93 = t18 * t91 * t5;
        let t97 = 1.0 + 0.027439371595564633 * t84 * t88 * t89 * t93;
        let t98 = pow_1_4(t97);
        let t100 = 1.0 - 1.0 / t98;
        let t103 = 1.0 + 1.0 * t100 * t80;
        let t104 = rmath::ln(t103);
        let t106 = t62 * t66 * t104;
        let tzk0 = -t32 + t57 + t106;
        zk[ip] += tzk0;
        let t108 = 1.0 / t7 / rho[ip];
        let t109 = t6 * t108;
        let t111 = t4 * t109 * t30;
        let t112 = 0.0011073470983333333 * t111;
        let t113 = t26 * t26;
        let t114 = 1.0 / t113;
        let t115 = t12 * t114;
        let t117 = 1.0 / t13 * t1;
        let t118 = t3 * t6;
        let t119 = t118 * t108;
        let t120 = t117 * t119;
        let t122 = t4 * t109;
        let t124 = rmath::sqrt(t10);
        let t125 = t124 * t1;
        let t126 = t125 * t119;
        let t131 = t20 * t5 / t21 / rho[ip];
        let t133 = -0.632975 * t120 - 0.29896666666666666 * t122 - 0.1023875 * t126 - 0.08215666666666667 * t131;
        let t134 = 1.0 / t29;
        let t135 = t133 * t134;
        let t136 = t115 * t135;
        let t137 = 1.0 * t136;
        let t138 = t43 * t1;
        let t141 = t138 * t118 * t108 * t54;
        let t142 = 0.00018311447306006544 * t141;
        let t143 = t43 * t45;
        let t144 = t50 * t50;
        let t145 = 1.0 / t144;
        let t150 = -0.8630833333333333 * t120 - 0.301925 * t122 - 0.05501625 * t126 - 0.082785 * t131;
        let t152 = 1.0 / t53;
        let t153 = t145 * t150 * t152;
        let t154 = t143 * t153;
        let t155 = 0.5848223622634646 * t154;
        let t157 = 1.0 / t98 / t97;
        let t158 = t85 * rho[ip];
        let t160 = 1.0 / t21 / t158;
        let t161 = t160 * t71;
        let t164 = t39 * t89;
        let t165 = t81 * sigma[ip] * t164;
        let t168 = t70 * t70;
        let t169 = 1.0 / t168;
        let t170 = t68 * t169;
        let t171 = t170 * t82;
        let t176 = t59 * t59;
        let t177 = 1.0 / t176;
        let t178 = t72 * t177;
        let t179 = t80 * t80;
        let t180 = 1.0 / t179;
        let t181 = t180 * sigma[ip];
        let t182 = t181 * t88;
        let t183 = t178 * t182;
        let t184 = t65 * t65;
        let t186 = 1.0 / t184 / t64;
        let t187 = t186 * t18;
        let t188 = t187 * t91;
        let t189 = t112 + t137 - t142 - t155;
        let t191 = t60 * t79;
        let t192 = t5 * t189 * t191;
        let t193 = t188 * t192;
        let t197 = 1.0 / t7 / t158;
        let t198 = t197 * t39;
        let t203 = -0.002743937159556463 * t161 * t73 * t165 + 0.004878720269691391 * t171 * sigma[ip] * t160 * t164 + 0.027439371595564633 * t183 * t193 - 0.0640252003896508 * t84 * t198 * t89 * t93;
        let t204 = t157 * t203;
        let t209 = t77 * t79;
        let t212 = 0.25 * t204 * t80 - 1.0 * t100 * t189 * t73 * t209;
        let t214 = 1.0 / t103;
        let t216 = t62 * t66 * t212 * t214;
        let tvrho0 = -t32 + t57 + t106 + rho[ip] * (t112 + t137 - t142 - t155 + t216);
        vrho[ip] += tvrho0;
        let t219 = t108 * t64;
        let t220 = t157 * t68;
        let t222 = t219 * t220 * t71;
        let t223 = t39 * t18;
        let t224 = t91 * t5;
        let t225 = t224 * t214;
        let t226 = t223 * t225;
        let tvsigma0 = 0.0006950474021161377 * t222 * t226;
        vsigma[ip] += tvsigma0;
    }
}
