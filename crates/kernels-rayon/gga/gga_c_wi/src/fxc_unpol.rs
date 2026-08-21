//! GGA_C_WI fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wi.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_wi_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_k: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = param_b * sigma[ip];
        let t2 = rho[ip] * rho[ip];
        let t3 = pow_1_3(rho[ip]);
        let t4 = t3 * t3;
        let t6 = 1.0 / t4 / t2;
        let t7 = param_k * sigma[ip];
        let t9 = rmath::exp(-t7 * t6);
        let t12 = t1 * t6 * t9 + param_a;
        let t13 = M_CBRT3;
        let t15 = pow_1_3(1.0 / M_PI);
        let t16 = t13 * t15;
        let t17 = M_CBRT4;
        let t18 = t17 * t17;
        let t22 = t13 * t13;
        let t23 = M_CBRTPI;
        let t25 = rmath::sqrt(sigma[ip]);
        let t26 = t25 * sigma[ip];
        let t27 = t2 * t2;
        let t28 = 1.0 / t27;
        let t31 = 1.0 / t3 / rho[ip];
        let t32 = t25 * t31;
        let t33 = rmath::sqrt(t32);
        let t38 = 1.0 + param_d * t17 * t22 * t23 * t33 * t26 * t28 / 3.0;
        let t42 = param_c + t16 * t18 / t3 * t38 / 4.0;
        let t43 = 1.0 / t42;
        let tzk0 = t12 * t43;
        zk[ip] += tzk0;
        let t44 = t2 * rho[ip];
        let t46 = 1.0 / t4 / t44;
        let t49 = sigma[ip] * sigma[ip];
        let t50 = param_b * t49;
        let t51 = t27 * t2;
        let t53 = 1.0 / t3 / t51;
        let t58 = 8.0 / 3.0 * t50 * t53 * param_k * t9 - 8.0 / 3.0 * t1 * t46 * t9;
        let t59 = rho[ip] * t58;
        let t61 = rho[ip] * t12;
        let t62 = t42 * t42;
        let t63 = 1.0 / t62;
        let t71 = t33 * sigma[ip] * t6;
        let t72 = t23 * t71;
        let t73 = t72 * t25;
        let t76 = -t16 * t18 * t31 * t38 / 12.0 - 14.0 / 3.0 * t15 * t6 * param_d * t73;
        let t77 = t63 * t76;
        let tvrho0 = t59 * t43 - t61 * t77 + tzk0;
        vrho[ip] += tvrho0;
        let t81 = t27 * rho[ip];
        let t83 = 1.0 / t3 / t81;
        let t87 = -t1 * t83 * param_k * t9 + param_b * t6 * t9;
        let t88 = rho[ip] * t87;
        let t90 = 1.0 / t4;
        let t91 = t90 * t12;
        let t92 = t63 * t15;
        let t93 = t91 * t92;
        let t94 = param_d * t23;
        let t95 = 1.0 / t25;
        let t96 = t71 * t95;
        let t97 = t94 * t96;
        let tvsigma0 = t88 * t43 - 7.0 / 4.0 * t93 * t97;
        vsigma[ip] += tvsigma0;
        let t102 = t12 * t63;
        let t106 = 1.0 / t4 / t27;
        let t110 = t27 * t44;
        let t112 = 1.0 / t3 / t110;
        let t117 = t49 * sigma[ip];
        let t118 = param_b * t117;
        let t119 = t27 * t27;
        let t120 = t119 * t2;
        let t121 = 1.0 / t120;
        let t122 = param_k * param_k;
        let t127 = 88.0 / 9.0 * t1 * t106 * t9 - 24.0 * t50 * t112 * param_k * t9 + 64.0 / 9.0 * t118 * t121 * t122 * t9;
        let t128 = rho[ip] * t127;
        let t133 = 1.0 / t62 / t42;
        let t134 = t76 * t76;
        let t135 = t133 * t134;
        let t148 = 1.0 / t81;
        let t151 = t33 * t32;
        let t152 = t23 * t151;
        let t153 = t152 * sigma[ip];
        let t156 = t16 * t18 / t3 / t2 * t38 / 9.0 + 14.0 * t15 * t46 * param_d * t73 + 140.0 / 9.0 * t15 * t148 * param_d * t153;
        let t157 = t63 * t156;
        let tv2rho20 = -2.0 * t102 * t76 + t128 * t43 + 2.0 * t61 * t135 - t61 * t157 + 2.0 * t58 * t43 - 2.0 * t59 * t77;
        v2rho2[ip] += tv2rho20;
        let t163 = param_b * t53;
        let t164 = t7 * t9;
        let t167 = t119 * rho[ip];
        let t168 = 1.0 / t167;
        let t173 = -8.0 / 3.0 * param_b * t46 * t9 + 8.0 * t163 * t164 - 8.0 / 3.0 * t50 * t168 * t122 * t9;
        let t174 = rho[ip] * t173;
        let t178 = 1.0 / t4 / rho[ip];
        let t179 = t178 * t12;
        let t180 = t179 * t92;
        let t183 = t90 * t58;
        let t184 = t183 * t92;
        let t187 = t133 * t15;
        let t188 = t91 * t187;
        let t190 = t94 * t96 * t76;
        let t193 = 1.0 / t44;
        let t194 = t193 * t12;
        let t196 = t15 * param_d;
        let t197 = t196 * t152;
        let tv2rhosigma0 = t87 * t43 + t174 * t43 - t88 * t77 + 7.0 / 6.0 * t180 * t97 - 7.0 / 4.0 * t184 * t97 + 7.0 / 2.0 * t188 * t190 + 35.0 / 6.0 * t194 * t63 * t197;
        v2rhosigma[ip] += tv2rhosigma0;
        let t201 = param_k * t9;
        let t204 = 1.0 / t119;
        let t208 = t1 * t204 * t122 * t9 - 2.0 * param_b * t83 * t201;
        let t209 = rho[ip] * t208;
        let t211 = t90 * t87;
        let t212 = t211 * t92;
        let t215 = t168 * t12;
        let t216 = t215 * t133;
        let t217 = t15 * t15;
        let t218 = param_d * param_d;
        let t219 = t217 * t218;
        let t220 = t23 * t23;
        let t222 = t219 * t220 * t26;
        let t225 = 1.0 / t2;
        let t226 = t225 * t12;
        let t227 = t226 * t92;
        let t228 = 1.0 / sigma[ip];
        let t229 = t151 * t228;
        let t230 = t94 * t229;
        let t233 = 1.0 / t26;
        let t234 = t71 * t233;
        let t235 = t94 * t234;
        let tv2sigma20 = t209 * t43 - 7.0 / 2.0 * t212 * t97 + 49.0 / 8.0 * t216 * t222 - 35.0 / 16.0 * t227 * t230 + 7.0 / 8.0 * t93 * t235;
        v2sigma2[ip] += tv2sigma20;
    }
}
