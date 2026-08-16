//! GGA_C_P86 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mdelta: f64,
    param_mgamma: f64,
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
        let t8 = 1.0 / t7;
        let t9 = t6 * t8;
        let t10 = t4 * t9;
        let t11 = t10 / 4.0;
        let t12 = 1.0 <= t11;
        let t13 = f64::sqrt(t10);
        let t16 = 1.0 + 0.52645e0 * t13 + 0.8335e-1 * t10;
        let t19 = f64::ln(t11);
        let t22 = t4 * t9 * t19;
        let t26 = piecewise3(t12, -0.1423e0 / t16, 0.311e-1 * t19 - 0.48e-1 + 0.5e-3 * t22 - 0.29e-2 * t10);
        let t29 = 1.0 + 0.69905e0 * t13 + 0.65275e-1 * t10;
        let t36 = piecewise3(t12, -0.843e-1 / t29, 0.1555e-1 * t19 - 0.269e-1 + 0.175e-3 * t22 - 0.12e-2 * t10);
        let t38 = 1.0 <= zeta_threshold;
        let t39 = pow_1_3(zeta_threshold);
        let t41 = piecewise3(t38, t39 * zeta_threshold, 1.0);
        let t43 = 2.0 * t41 - 2.0;
        let t45 = M_CBRT2;
        let t48 = 1.0 / (2.0 * t45 - 2.0);
        let t49 = (t36 - t26) * t43 * t48;
        let t50 = rho[ip] * rho[ip];
        let t52 = 1.0 / t7 / t50;
        let t53 = sigma[ip] * t52;
        let t54 = param_aa + param_bb;
        let t55 = param_ftilde * t54;
        let t56 = param_malpha * t1;
        let t57 = t3 * t6;
        let t58 = t57 * t8;
        let t61 = t1 * t1;
        let t62 = param_mbeta * t61;
        let t63 = t3 * t3;
        let t64 = t63 * t5;
        let t65 = t7 * t7;
        let t66 = 1.0 / t65;
        let t67 = t64 * t66;
        let t70 = param_bb + t56 * t58 / 4.0 + t62 * t67 / 4.0;
        let t71 = param_mgamma * t1;
        let t74 = param_mdelta * t61;
        let t77 = 1.0 / rho[ip];
        let t80 = 1.0 + t71 * t58 / 4.0 + t74 * t67 / 4.0 + 0.23873241463784300365e4 * param_mbeta * t77;
        let t81 = 1.0 / t80;
        let t83 = t70 * t81 + param_aa;
        let t84 = 1.0 / t83;
        let t85 = f64::sqrt(sigma[ip]);
        let t86 = t84 * t85;
        let t87 = f64::powf(rho[ip], 1.0 / 6.0);
        let t89 = 1.0 / t87 / rho[ip];
        let t92 = f64::exp(-t55 * t86 * t89);
        let t94 = t39 * t39;
        let t96 = piecewise3(t38, t94 * zeta_threshold, 1.0);
        let t97 = f64::sqrt(t96);
        let t98 = 1.0 / t97;
        let t99 = t92 * t83 * t98;
        let t100 = t53 * t99;
        let tzk0 = t26 + t49 + t100;
        zk[ip] += tzk0;
        let t101 = t16 * t16;
        let t102 = 1.0 / t101;
        let t104 = 1.0 / t13 * t1;
        let t106 = 1.0 / t7 / rho[ip];
        let t107 = t57 * t106;
        let t108 = t104 * t107;
        let t110 = t6 * t106;
        let t111 = t4 * t110;
        let t113 = -0.87741666666666666667e-1 * t108 - 0.27783333333333333333e-1 * t111;
        let t118 = t4 * t110 * t19;
        let t122 = piecewise3(t12, 0.1423e0 * t102 * t113, -0.10366666666666666667e-1 * t77 - 0.16666666666666666667e-3 * t118 + 0.8e-3 * t111);
        let t123 = t29 * t29;
        let t124 = 1.0 / t123;
        let t127 = -0.11650833333333333333e0 * t108 - 0.21758333333333333333e-1 * t111;
        let t134 = piecewise3(t12, 0.843e-1 * t124 * t127, -0.51833333333333333333e-2 * t77 - 0.58333333333333333333e-4 * t118 + 0.34166666666666666667e-3 * t111);
        let t137 = (t134 - t122) * t43 * t48;
        let t138 = t50 * rho[ip];
        let t140 = 1.0 / t7 / t138;
        let t141 = sigma[ip] * t140;
        let t142 = t141 * t99;
        let t144 = t83 * t83;
        let t145 = 1.0 / t144;
        let t146 = t55 * t145;
        let t147 = t85 * t89;
        let t152 = t64 / t65 / rho[ip];
        let t155 = -t56 * t107 / 12.0 - t62 * t152 / 6.0;
        let t157 = t80 * t80;
        let t158 = 1.0 / t157;
        let t159 = t70 * t158;
        let t164 = 1.0 / t50;
        let t167 = -t71 * t107 / 12.0 - t74 * t152 / 6.0 - 0.23873241463784300365e4 * param_mbeta * t164;
        let t169 = t155 * t81 - t159 * t167;
        let t173 = 1.0 / t87 / t50;
        let t177 = t146 * t147 * t169 + 7.0 / 6.0 * t55 * t86 * t173;
        let t178 = t53 * t177;
        let t179 = t178 * t99;
        let t181 = t92 * t169 * t98;
        let t182 = t53 * t181;
        let tvrho0 = t26 + t49 + t100 + rho[ip] * (t122 + t137 - 7.0 / 3.0 * t142 + t179 + t182);
        vrho[ip] += tvrho0;
        let t185 = t52 * t92;
        let t186 = t83 * t98;
        let t187 = t185 * t186;
        let t188 = f64::sqrt(rho[ip]);
        let t190 = 1.0 / t188 / t138;
        let t191 = t85 * t190;
        let t192 = t191 * param_ftilde;
        let t194 = t54 * t92 * t98;
        let t196 = t192 * t194 / 2.0;
        let tvsigma0 = rho[ip] * (t187 - t196);
        vsigma[ip] += tvsigma0;
    }
}
