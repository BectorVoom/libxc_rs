//! GGA_X_PBE_ERF_GWS exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe_erf_gws.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbe_erf_gws_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_ax: f64,
    param_b_PBE: f64,
    param_kappa: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = param_hyb_omega_0 * param_hyb_omega_0;
        let t4 = param_ax * t3;
        let t5 = M_CBRT3;
        let t7 = M_CBRTPI;
        let t8 = t7 * M_PI;
        let t9 = 1.0 / t8;
        let t10 = 2.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t12 = M_CBRT2;
        let t13 = piecewise3(t10, t11, t12);
        let t14 = t13 * t13;
        let t15 = 1.0 / t14;
        let t16 = t9 * t15;
        let t17 = t12 * t12;
        let t18 = pow_1_3(rho[ip]);
        let t19 = t18 * t18;
        let t20 = 1.0 / t19;
        let t25 = f64::exp(-t4 * t5 * t16 * t17 * t20 / 12.0);
        let t26 = param_b_PBE * t25;
        let t27 = t26 * sigma[ip];
        let t28 = param_kappa + 1.0;
        let t29 = t5 * t28;
        let t30 = t5 * t5;
        let t31 = t12 * t30;
        let t32 = t7 * t7;
        let t34 = t31 / t32;
        let t35 = 1.0 / t18;
        let t37 = 1.0 / t13;
        let t40 = t34 * param_hyb_omega_0 * t35 * t37 / 6.0;
        let t41 = t40 < 0.05;
        let t42 = t14 * t14;
        let t43 = M_PI * M_PI;
        let t44 = t32 * t43;
        let t45 = t42 * t44;
        let t46 = t18 * rho[ip];
        let t47 = t45 * t46;
        let t49 = t14 * t8;
        let t50 = t49 * t17;
        let t52 = t5 * t19 * t3;
        let t53 = t50 * t52;
        let t55 = 7.0 * t47 - 6.0 * t53;
        let t56 = t14 * t13;
        let t57 = 1.0 / param_hyb_omega_0;
        let t64 = erf_approx(t57 * t5 * t32 * t13 * t17 * t18 / 2.0);
        let t66 = f64::sqrt(M_PI);
        let t67 = t66 * t43;
        let t68 = t56 * t64 * t67;
        let t69 = rho[ip] * param_hyb_omega_0;
        let t75 = t3 * t3;
        let t76 = t75 * t30;
        let t78 = 12.0 * t76 * t12;
        let t79 = -36.0 * t68 * t31 * t69 + 81.0 * t47 + 54.0 * t53 - t78;
        let t80 = 1.0 / t79;
        let t82 = 10000000000.0 < t40;
        let t83 = t43 * t43;
        let t84 = rho[ip] * rho[ip];
        let t86 = t42 * t14;
        let t90 = t44 * t17 * t5;
        let t96 = t8 * t12 * t30;
        let t102 = t75 * t3;
        let t103 = 1.0 / t102;
        let t107 = 1.0 / t3;
        let t108 = t107 * t30;
        let t113 = t108 * t8 * t14 * t12 * t19 / 2.0;
        let t114 = f64::exp(t113);
        let t115 = t114 * t8;
        let t118 = t5 * t3;
        let t119 = t14 * t17 * t118;
        let t123 = t114 * t12;
        let t127 = (7.0 * t115 * t19 * t119 - 12.0 * t123 * t76 + 6.0 * t47 + 11.0 * t53 + t78) * t8;
        let t128 = t19 * t14;
        let t129 = t127 * t128;
        let t130 = t17 * t30;
        let t131 = t42 * t114;
        let t132 = t44 * t12;
        let t136 = t56 * t114;
        let t143 = t14 * t114 * t8;
        let t148 = t114 * t17;
        let t153 = 12.0 * t136 * t64 * t67 * t130 * t69 - 27.0 * t131 * t132 * t46 - 4.0 * t130 * t75 - 36.0 * t143 * t52 + 4.0 * t148 * t76 + 24.0 * t49 * t52;
        let t156 = t130 * t107 / t153;
        let t159 = piecewise5(t41, t55 * t80, t82, (5600.0 * t96 * t19 * t75 * t14 - 140.0 * t90 * t46 * t3 * t42 - 1863.0 * t83 * t84 * t86) * t103 / 201600.0, -t129 * t156 / 18.0);
        let t163 = t19 * t84;
        let t165 = param_kappa * t163 * t8;
        let t166 = 27.0 / 28.0 * t27 * t29 * t159 + t165;
        let t167 = t166 * t46;
        let t170 = piecewise3(t10, t11 * zeta_threshold, 2.0 * t12);
        let t171 = t170 * t17;
        let t172 = t167 * t171;
        let t173 = 1.35 <= t40;
        let t174 = 1.35 < t40;
        let t175 = piecewise3(t174, t40, 1.35);
        let t176 = t175 * t175;
        let t177 = t176 * t176;
        let t178 = t177 * t176;
        let t179 = t177 * t177;
        let t182 = t179 * t177;
        let t184 = t179 * t176;
        let t190 = 24088884019200.0 * t179 * t178 + 19448.0 * t176 - 807840.0 * t177 + 30551040.0 * t178 - 1045524480.0 * t179 - 903333150720.0 * t182 + 32261898240.0 * t184 - 429.0;
        let t191 = t179 * t179;
        let t192 = 1.0 / t191;
        let t195 = piecewise3(t174, 1.35, t40);
        let t196 = t195 * t195;
        let t197 = t196 * t196;
        let t200 = 32.0 * t197 - 16.0 * t196;
        let t203 = f64::exp(-1.0 / t196 / 4.0);
        let t207 = 1.0 / t195;
        let t209 = erf_approx(t207 / 2.0);
        let t210 = t66 * t209;
        let t215 = piecewise3(t173, t190 * t192 / 867199824691200.0, t200 * t203 / 3.0 - 32.0 / 3.0 * t197 - 8.0 / 3.0 * t210 * t195 + 8.0 * t196 + 1.0);
        let t216 = 1.0 / t7;
        let t217 = t215 * t216;
        let t218 = param_b_PBE * t159;
        let t220 = t25 * sigma[ip] * t5;
        let t224 = 864.0 * t218 * t220 + 896.0 * t165;
        let t225 = 1.0 / t224;
        let t226 = t5 * t225;
        let t227 = t217 * t226;
        let t230 = piecewise3(t2, 0.0, -84.0 * t172 * t227);
        let t231 = 1.0 / rho[ip];
        let tzk0 = 2.0 * t230 * t231;
        zk[ip] += tzk0;
    }
}
