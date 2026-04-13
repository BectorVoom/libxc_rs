//! GGA_C_SOGGA11 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_c_sogga11_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_sogga11_a_0: f64,
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_0: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t10 = t4 * t6 / t7;
        let t12 = 1.0 + 0.53425e-1 * t10;
        let t13 = f64::sqrt(t10);
        let t16 = pow_3_2(t10);
        let t18 = t1 * t1;
        let t19 = t3 * t3;
        let t20 = t18 * t19;
        let t21 = t7 * t7;
        let t24 = t20 * t5 / t21;
        let t26 = 0.379785e1 * t13 + 0.8969e0 * t10 + 0.204775e0 * t16 + 0.123235e0 * t24;
        let t29 = 1.0 + 0.16081979498692535067e2 / t26;
        let t30 = f64::ln(t29);
        let t33 = 1.0 <= zeta_threshold;
        let t34 = pow_1_3(zeta_threshold);
        let t36 = piecewise3(t33, t34 * zeta_threshold, 1.0);
        let t39 = M_CBRT2;
        let t43 = (2.0 * t36 - 2.0) / (2.0 * t39 - 2.0);
        let t45 = 1.0 + 0.278125e-1 * t10;
        let t50 = 0.51785e1 * t13 + 0.905775e0 * t10 + 0.1100325e0 * t16 + 0.1241775e0 * t24;
        let t53 = 1.0 + 0.29608749977793437516e2 / t50;
        let t54 = f64::ln(t53);
        let t58 = -0.621814e-1 * t12 * t30 + 0.19751673498613801407e-1 * t43 * t45 * t54;
        let t60 = param_sogga11_a_1;
        let t61 = t34 * t34;
        let t62 = piecewise3(t33, t61, 1.0);
        let t63 = t39 * t62;
        let t64 = rho[ip] * rho[ip];
        let t66 = 1.0 / t7 / t64;
        let t67 = sigma[ip] * t66;
        let t68 = t63 * t67;
        let t69 = 1.0 / t3;
        let t70 = t18 * t69;
        let t71 = 1.0 / t58;
        let t72 = t5 * t71;
        let t73 = t70 * t72;
        let t75 = 0.69506584583333333332e-3 * t68 * t73;
        let t76 = 1.0 - t75;
        let t78 = 1.0 - 1.0 / t76;
        let t80 = param_sogga11_a_2;
        let t81 = t78 * t78;
        let t83 = param_sogga11_a_3;
        let t84 = t81 * t78;
        let t86 = param_sogga11_a_4;
        let t87 = t81 * t81;
        let t89 = param_sogga11_a_5;
        let t93 = param_sogga11_b_1;
        let t94 = f64::exp(t75);
        let t95 = 1.0 - t94;
        let t97 = param_sogga11_b_2;
        let t98 = t95 * t95;
        let t100 = param_sogga11_b_3;
        let t101 = t98 * t95;
        let t103 = param_sogga11_b_4;
        let t104 = t98 * t98;
        let t106 = param_sogga11_b_5;
        let t109 = t106 * t104 * t95 + t89 * t87 * t78 + t100 * t101 + t103 * t104 + t60 * t78 + t80 * t81 + t83 * t84 + t86 * t87 + t93 * t95 + t97 * t98 + param_sogga11_a_0 + param_sogga11_b_0;
        let tzk0 = t58 * t109;
        zk[ip] += tzk0;
        let t111 = 1.0 / t7 / rho[ip];
        let t112 = t6 * t111;
        let t116 = t26 * t26;
        let t117 = 1.0 / t116;
        let t118 = t12 * t117;
        let t120 = 1.0 / t13 * t1;
        let t121 = t3 * t6;
        let t122 = t121 * t111;
        let t123 = t120 * t122;
        let t125 = t4 * t112;
        let t127 = f64::sqrt(t10);
        let t128 = t127 * t1;
        let t129 = t128 * t122;
        let t134 = t20 * t5 / t21 / rho[ip];
        let t136 = -0.632975e0 * t123 - 0.29896666666666666667e0 * t125 - 0.1023875e0 * t129 - 0.82156666666666666667e-1 * t134;
        let t137 = 1.0 / t29;
        let t138 = t136 * t137;
        let t141 = t43 * t1;
        let t146 = t43 * t45;
        let t147 = t50 * t50;
        let t148 = 1.0 / t147;
        let t153 = -0.86308333333333333334e0 * t123 - 0.301925e0 * t125 - 0.5501625e-1 * t129 - 0.82785e-1 * t134;
        let t155 = 1.0 / t53;
        let t156 = t148 * t153 * t155;
        let t159 = 0.11073470983333333333e-2 * t4 * t112 * t30 + 1.0 * t118 * t138 - 0.18311447306006545054e-3 * t141 * t121 * t111 * t54 - 0.5848223622634646207e0 * t146 * t156;
        let t160 = rho[ip] * t159;
        let t162 = rho[ip] * t58;
        let t163 = t76 * t76;
        let t164 = 1.0 / t163;
        let t165 = t60 * t164;
        let t166 = t64 * rho[ip];
        let t168 = 1.0 / t7 / t166;
        let t169 = sigma[ip] * t168;
        let t170 = t63 * t169;
        let t173 = t58 * t58;
        let t174 = 1.0 / t173;
        let t175 = t5 * t174;
        let t176 = t175 * t159;
        let t177 = t70 * t176;
        let t180 = 0.16218203069444444444e-2 * t170 * t73 + 0.69506584583333333332e-3 * t68 * t177;
        let t182 = t80 * t78;
        let t183 = t164 * t180;
        let t186 = t83 * t81;
        let t189 = t86 * t84;
        let t192 = t89 * t87;
        let t195 = -t180;
        let t196 = t93 * t195;
        let t198 = t97 * t95;
        let t199 = t195 * t94;
        let t202 = t100 * t98;
        let t205 = t103 * t101;
        let t208 = t106 * t104;
        let t211 = t165 * t180 + 2.0 * t182 * t183 + 3.0 * t186 * t183 + 4.0 * t189 * t183 + 5.0 * t192 * t183 - t196 * t94 - 2.0 * t198 * t199 - 3.0 * t202 * t199 - 4.0 * t205 * t199 - 5.0 * t208 * t199;
        let tvrho0 = t160 * t109 + t162 * t211 + tzk0;
        vrho[ip] += tvrho0;
        let t213 = t165 * t63;
        let t214 = t66 * t18;
        let t215 = t69 * t5;
        let t216 = t215 * t71;
        let t217 = t214 * t216;
        let t221 = t164 * t39 * t62;
        let t222 = t182 * t221;
        let t225 = t186 * t221;
        let t228 = t189 * t221;
        let t231 = t192 * t221;
        let t234 = t93 * t39;
        let t235 = t62 * t66;
        let t238 = t70 * t72 * t94;
        let t241 = t63 * t66;
        let t242 = t198 * t241;
        let t245 = t202 * t241;
        let t248 = t205 * t241;
        let t251 = t208 * t241;
        let t254 = -0.69506584583333333332e-3 * t213 * t217 - 0.13901316916666666666e-2 * t222 * t217 - 0.20851975375e-2 * t225 * t217 - 0.27802633833333333333e-2 * t228 * t217 - 0.34753292291666666666e-2 * t231 * t217 - 0.69506584583333333332e-3 * t234 * t235 * t238 - 0.13901316916666666666e-2 * t242 * t238 - 0.20851975375e-2 * t245 * t238 - 0.27802633833333333333e-2 * t248 * t238 - 0.34753292291666666666e-2 * t251 * t238;
        let tvsigma0 = t162 * t254;
        vsigma[ip] += tvsigma0;
    }
}
