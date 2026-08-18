//! MGGA_C_RMGGAC exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rmggac.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_rmggac_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let t2 = M_CBRT3;
        let t3 = 1.0 / M_PI;
        let t4 = pow_1_3(t3);
        let t5 = t2 * t4;
        let t6 = M_CBRT4;
        let t7 = t6 * t6;
        let t8 = rho0 + rho1;
        let t9 = pow_1_3(t8);
        let t12 = t5 * t7 / t9;
        let t13 = f64::sqrt(t12);
        let t16 = 1.0 + 0.04445 * t13 + 0.03138525 * t12;
        let t17 = 1.0 / t16;
        let t20 = f64::exp(1.0 * t17);
        let t21 = t20 - 1.0;
        let t22 = M_CBRT6;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = t24 * t24;
        let t26 = 1.0 / t25;
        let t27 = t22 * t26;
        let t28 = M_CBRT2;
        let t29 = t28 * t28;
        let t31 = sigma0 + 2.0 * sigma1 + sigma2;
        let t32 = t29 * t31;
        let t33 = t8 * t8;
        let t34 = t9 * t9;
        let t36 = 1.0 / t34 / t33;
        let t38 = t27 * t32 * t36;
        let t40 = 1.0 + 0.02133764210437636 * t38;
        let t41 = pow_1_4(t40);
        let t43 = 1.0 - 1.0 / t41;
        let t45 = t21 * t43 + 1.0;
        let t46 = f64::ln(t45);
        let t48 = -0.0285764 * t17 + 0.0285764 * t46;
        let t49 = t28 - 1.0;
        let t50 = rho0 - rho1;
        let t51 = 1.0 / t8;
        let t52 = t50 * t51;
        let t53 = 1.0 + t52;
        let t54 = t53 <= zeta_threshold;
        let t55 = pow_1_3(zeta_threshold);
        let t56 = t55 * zeta_threshold;
        let t57 = pow_1_3(t53);
        let t58 = t57 * t53;
        let t59 = piecewise3(t54, t56, t58);
        let t60 = 1.0 - t52;
        let t61 = t60 <= zeta_threshold;
        let t62 = pow_1_3(t60);
        let t63 = t62 * t60;
        let t64 = piecewise3(t61, t56, t63);
        let t65 = t59 + t64 - 2.0;
        let t68 = 1.0 / t49 / 2.0;
        let t71 = 1.0 - 2.363 * t49 * t65 * t68;
        let t72 = t48 * t71;
        let t73 = t50 * t50;
        let t74 = t73 * t73;
        let t75 = t74 * t74;
        let t76 = t75 * t74;
        let t77 = t33 * t33;
        let t78 = t77 * t77;
        let t79 = t78 * t77;
        let t80 = 1.0 / t79;
        let t82 = -t76 * t80 + 1.0;
        let t83 = pow_1_3(rho0);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / rho0;
        let t87 = tau0 * t86;
        let t88 = t53 / 2.0;
        let t89 = pow_1_3(t88);
        let t90 = t89 * t89;
        let t91 = t90 * t88;
        let t94 = pow_1_3(rho1);
        let t95 = t94 * t94;
        let t97 = 1.0 / t95 / rho1;
        let t98 = tau1 * t97;
        let t99 = t60 / 2.0;
        let t100 = pow_1_3(t99);
        let t101 = t100 * t100;
        let t102 = t101 * t99;
        let t107 = 2.0 * t87 * t91 + 2.0 * t98 * t102 - t31 * t36 / 4.0;
        let t108 = t107 * t107;
        let t109 = t108 * t107;
        let t114 = 0.08 + 5.0 / 18.0 * t107 * t29 * t27 + 0.0125 * t38;
        let t115 = t114 * t114;
        let t116 = t115 * t114;
        let t117 = 1.0 / t116;
        let t118 = t109 * t117;
        let t120 = t108 * t108;
        let t121 = t120 * t108;
        let t122 = t115 * t115;
        let t124 = 1.0 / t122 / t115;
        let t127 = 1.0 + 0.006652356501035449 * t118 + 4.42538470168686e-05 * t121 * t124;
        let t128 = 1.0 / t127;
        let t129 = t118 * t128;
        let t131 = 1.0 - 0.01995706950310635 * t129;
        let t132 = t82 * t131;
        let t133 = t72 * t132;
        let t135 = 1.0 + 0.053425 * t12;
        let t138 = pow_3_2(t12);
        let t140 = t2 * t2;
        let t141 = t4 * t4;
        let t142 = t140 * t141;
        let t145 = t142 * t6 / t34;
        let t147 = 3.79785 * t13 + 0.8969 * t12 + 0.204775 * t138 + 0.123235 * t145;
        let t150 = 1.0 + 16.081979498692537 / t147;
        let t151 = f64::ln(t150);
        let t153 = 0.0621814 * t135 * t151;
        let t154 = 1.0 / t77;
        let t155 = t74 * t154;
        let t156 = t65 * t68;
        let t158 = 1.0 + 0.05137 * t12;
        let t163 = 7.05945 * t13 + 1.549425 * t12 + 0.420775 * t138 + 0.1562925 * t145;
        let t166 = 1.0 + 32.16395899738507 / t163;
        let t167 = f64::ln(t166);
        let t171 = 1.0 + 0.0278125 * t12;
        let t176 = 5.1785 * t13 + 0.905775 * t12 + 0.1100325 * t138 + 0.1241775 * t145;
        let t179 = 1.0 + 29.608749977793437 / t176;
        let t180 = f64::ln(t179);
        let t181 = t171 * t180;
        let t183 = -0.0310907 * t158 * t167 + t153 - 0.0197516734986138 * t181;
        let t184 = t156 * t183;
        let t185 = t155 * t184;
        let t187 = 0.0197516734986138 * t156 * t181;
        let t188 = t55 * t55;
        let t189 = t57 * t57;
        let t190 = piecewise3(t54, t188, t189);
        let t191 = t62 * t62;
        let t192 = piecewise3(t61, t188, t191);
        let t194 = t190 / 2.0 + t192 / 2.0;
        let t195 = t194 * t194;
        let t196 = t195 * t194;
        let t197 = -t153 + t185 + t187;
        let t198 = 1.0 / t196;
        let t201 = f64::exp(-32.16364864430221 * t197 * t198);
        let t202 = t201 - 1.0;
        let t203 = f64::ln(2.0);
        let t204 = 1.0 - t203;
        let t205 = 1.0 / t204;
        let t206 = t197 * t205;
        let t207 = t23 * t198;
        let t209 = f64::exp(-t206 * t207);
        let t210 = t209 - 1.0;
        let t211 = 1.0 / t210;
        let t212 = t205 * t211;
        let t214 = 1.0 / t9 / t33;
        let t215 = t31 * t214;
        let t217 = 1.0 / t195;
        let t219 = 1.0 / t4;
        let t220 = t140 * t219;
        let t221 = t220 * t6;
        let t222 = t28 * t217 * t221;
        let t225 = 1.0 + 0.02743955640261198 * t212 * t215 * t222;
        let t226 = pow_1_4(t225);
        let t228 = 1.0 - 1.0 / t226;
        let t230 = t202 * t228 + 1.0;
        let t231 = f64::ln(t230);
        let t234 = -t153 + t185 + t187 + 0.031091 * t196 * t231;
        let t235 = t234 * t109;
        let t236 = t117 * t128;
        let t238 = 0.01995706950310635 * t235 * t236;
        let tzk0 = t133 + t238;
        zk[ip] += tzk0;
    }
}
