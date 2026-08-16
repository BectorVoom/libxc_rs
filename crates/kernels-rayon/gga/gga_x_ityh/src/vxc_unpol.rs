//! GGA_X_ITYH vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t20 * t24;
        let t28 = t27 * t25;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / t32;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t36 * t29;
        let t39 = 1.0 / t19 / rho[ip];
        let t41 = f64::ln(t37 * t39 + f64::sqrt(pow_2(t37 * t39) + 1.0));
        let t42 = t39 * t41;
        let t45 = 1.0 + 0.252e-1 * t37 * t42;
        let t46 = 1.0 / t45;
        let t51 = 1.0 + 0.93333333333333333332e-3 * t28 * t31 * t35 * t46;
        let t54 = M_PI * t20 * t26 / t51;
        let t55 = f64::sqrt(t54);
        let t57 = param_hyb_omega_0 / t55;
        let t58 = t11 * rho[ip];
        let t59 = pow_1_3(t58);
        let t60 = 1.0 / t59;
        let t61 = t29 * t60;
        let t63 = t57 * t61 / 2.0;
        let t64 = 0.135e1 <= t63;
        let t65 = 0.135e1 < t63;
        let t66 = piecewise3(t65, t63, 0.135e1);
        let t67 = t66 * t66;
        let t70 = t67 * t67;
        let t71 = 1.0 / t70;
        let t73 = t70 * t67;
        let t74 = 1.0 / t73;
        let t76 = t70 * t70;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t67;
        let t83 = 1.0 / t76 / t70;
        let t86 = 1.0 / t76 / t73;
        let t88 = t76 * t76;
        let t89 = 1.0 / t88;
        let t92 = piecewise3(t65, 0.135e1, t63);
        let t93 = f64::sqrt(M_PI);
        let t94 = 1.0 / t92;
        let t96 = erf_approx(t94 / 2.0);
        let t98 = t92 * t92;
        let t99 = 1.0 / t98;
        let t101 = f64::exp(-t99 / 4.0);
        let t102 = t101 - 1.0;
        let t105 = t101 - 3.0 / 2.0 - 2.0 * t98 * t102;
        let t108 = 2.0 * t92 * t105 + t93 * t96;
        let t112 = piecewise3(t64, 1.0 / t67 / 36.0 - t71 / 960.0 + t74 / 26880.0 - t77 / 829440.0 + t80 / 28385280.0 - t83 / 0.107347968e10 + t86 / 0.445906944e11 - t89 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t92 * t108);
        let t113 = t19 * t112;
        let t117 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t113 * t51);
        let tzk0 = 2.0 * t117;
        zk[ip] += tzk0;
        let t118 = 1.0 / t33;
        let t119 = t118 * t112;
        let t123 = t67 * t66;
        let t124 = 1.0 / t123;
        let t127 = param_hyb_omega_0 / t55 / t54;
        let t129 = t127 * t61 * M_PI;
        let t130 = t51 * t51;
        let t131 = 1.0 / t130;
        let t132 = t25 * t131;
        let t133 = t32 * rho[ip];
        let t135 = 1.0 / t33 / t133;
        let t140 = t25 * sigma[ip];
        let t141 = t27 * t140;
        let t142 = t30 * t35;
        let t143 = t45 * t45;
        let t144 = 1.0 / t143;
        let t147 = 1.0 / t19 / t32 * t41;
        let t151 = t31 * t35 + 1.0;
        let t152 = f64::sqrt(t151);
        let t153 = 1.0 / t152;
        let t154 = t135 * t153;
        let t157 = -0.336e-1 * t37 * t147 - 0.336e-1 * t31 * t154;
        let t158 = t144 * t157;
        let t159 = t142 * t158;
        let t162 = -0.24888888888888888889e-2 * t28 * t31 * t135 * t46 - 0.93333333333333333332e-3 * t141 * t159;
        let t168 = 1.0 / t59 / t58;
        let t169 = t29 * t168;
        let t173 = t129 * t27 * t132 * t162 / 4.0 - t57 * t169 * t11 / 6.0;
        let t174 = piecewise3(t65, t173, 0.0);
        let t177 = t70 * t66;
        let t178 = 1.0 / t177;
        let t181 = t70 * t123;
        let t182 = 1.0 / t181;
        let t186 = 1.0 / t76 / t66;
        let t190 = 1.0 / t76 / t123;
        let t194 = 1.0 / t76 / t177;
        let t198 = 1.0 / t76 / t181;
        let t202 = 1.0 / t88 / t66;
        let t206 = piecewise3(t65, 0.0, t173);
        let t208 = t101 * t99;
        let t212 = t98 * t92;
        let t213 = 1.0 / t212;
        let t217 = t92 * t102;
        let t222 = t213 * t206 * t101 / 2.0 - 4.0 * t217 * t206 - t94 * t206 * t101;
        let t225 = 2.0 * t206 * t105 - t208 * t206 + 2.0 * t92 * t222;
        let t229 = piecewise3(t64, -t124 * t174 / 18.0 + t178 * t174 / 240.0 - t182 * t174 / 4480.0 + t186 * t174 / 103680.0 - t190 * t174 / 2838528.0 + t194 * t174 / 89456640.0 - t198 * t174 / 0.31850496e10 + t202 * t174 / 0.1263403008e12, -8.0 / 3.0 * t206 * t108 - 8.0 / 3.0 * t92 * t225);
        let t230 = t19 * t229;
        let t238 = piecewise3(t2, 0.0, -t18 * t119 * t51 / 8.0 - 3.0 / 8.0 * t18 * t230 * t51 - 3.0 / 8.0 * t18 * t113 * t162);
        let tvrho0 = 2.0 * rho[ip] * t238 + 2.0 * t117;
        vrho[ip] += tvrho0;
        let t245 = 1.0 / t36 * t29;
        let t250 = 0.126e-1 * t245 * t42 + 0.126e-1 * t142 * t153;
        let t251 = t144 * t250;
        let t252 = t142 * t251;
        let t255 = 0.93333333333333333332e-3 * t28 * t142 * t46 - 0.93333333333333333332e-3 * t141 * t252;
        let t259 = t129 * t27 * t132 * t255 / 4.0;
        let t260 = piecewise3(t65, t259, 0.0);
        let t263 = t178 * t260;
        let t265 = t182 * t260;
        let t267 = t186 * t260;
        let t269 = t190 * t260;
        let t271 = t194 * t260;
        let t273 = t198 * t260;
        let t275 = t202 * t260;
        let t278 = piecewise3(t65, 0.0, t259);
        let t290 = t213 * t278 * t101 / 2.0 - 4.0 * t217 * t278 - t94 * t278 * t101;
        let t293 = 2.0 * t278 * t105 - t208 * t278 + 2.0 * t92 * t290;
        let t297 = piecewise3(t64, -t124 * t260 / 18.0 + t263 / 240.0 - t265 / 4480.0 + t267 / 103680.0 - t269 / 2838528.0 + t271 / 89456640.0 - t273 / 0.31850496e10 + t275 / 0.1263403008e12, -8.0 / 3.0 * t278 * t108 - 8.0 / 3.0 * t92 * t293);
        let t298 = t19 * t297;
        let t305 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t113 * t255 - 3.0 / 8.0 * t18 * t298 * t51);
        let tvsigma0 = 2.0 * rho[ip] * t305;
        vsigma[ip] += tvsigma0;
    }
}
