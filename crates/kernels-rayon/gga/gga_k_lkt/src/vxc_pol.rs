//! GGA_K_LKT vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lkt_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = M_CBRT6;
        let t33 = t32 * t32;
        let t34 = M_PI * M_PI;
        let t35 = pow_1_3(t34);
        let t37 = t33 / t35;
        let t38 = f64::sqrt(sigma0);
        let t39 = pow_1_3(rho0);
        let t41 = 1.0 / t39 / rho0;
        let t44 = t37 * t38 * t41 / 12.0;
        let t45 = t44 < 200.0;
        let t46 = piecewise3(t45, t44, 200.0);
        let t47 = param_a * t46;
        let t48 = f64::cosh(t47);
        let t49 = 1.0 / t48;
        let t50 = t35 * t35;
        let t52 = t32 / t50;
        let t53 = rho0 * rho0;
        let t54 = t39 * t39;
        let t56 = 1.0 / t54 / t53;
        let t60 = t49 + 5.0 / 72.0 * t52 * sigma0 * t56;
        let t64 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t60);
        let t65 = rho1 <= dens_threshold;
        let t66 = -t17;
        let t68 = piecewise5(t15, t12, t11, t16, t66 * t8);
        let t69 = 1.0 + t68;
        let t70 = t69 <= zeta_threshold;
        let t71 = pow_1_3(t69);
        let t72 = t71 * t71;
        let t74 = piecewise3(t70, t24, t72 * t69);
        let t75 = t74 * t30;
        let t76 = f64::sqrt(sigma2);
        let t77 = pow_1_3(rho1);
        let t79 = 1.0 / t77 / rho1;
        let t82 = t37 * t76 * t79 / 12.0;
        let t83 = t82 < 200.0;
        let t84 = piecewise3(t83, t82, 200.0);
        let t85 = param_a * t84;
        let t86 = f64::cosh(t85);
        let t87 = 1.0 / t86;
        let t88 = rho1 * rho1;
        let t89 = t77 * t77;
        let t91 = 1.0 / t89 / t88;
        let t95 = t87 + 5.0 / 72.0 * t52 * sigma2 * t91;
        let t99 = piecewise3(t65, 0.0, 3.0 / 20.0 * t6 * t75 * t95);
        let tzk0 = t64 + t99;
        zk[ip] += tzk0;
        let t100 = t7 * t7;
        let t101 = 1.0 / t100;
        let t102 = t17 * t101;
        let t104 = piecewise5(t11, 0.0, t15, 0.0, t8 - t102);
        let t107 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t104);
        let t108 = t107 * t30;
        let t112 = 1.0 / t29;
        let t113 = t28 * t112;
        let t116 = t6 * t113 * t60 / 10.0;
        let t117 = t48 * t48;
        let t118 = 1.0 / t117;
        let t119 = t118 * param_a;
        let t121 = 1.0 / t39 / t53;
        let t125 = piecewise3(t45, -t37 * t38 * t121 / 9.0, 0.0);
        let t126 = f64::sinh(t47);
        let t127 = t125 * t126;
        let t129 = t53 * rho0;
        let t131 = 1.0 / t54 / t129;
        let t135 = -t119 * t127 - 5.0 / 27.0 * t52 * sigma0 * t131;
        let t140 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t108 * t60 + t116 + 3.0 / 20.0 * t6 * t31 * t135);
        let t141 = t66 * t101;
        let t143 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t141);
        let t146 = piecewise3(t70, 0.0, 5.0 / 3.0 * t72 * t143);
        let t147 = t146 * t30;
        let t151 = t74 * t112;
        let t154 = t6 * t151 * t95 / 10.0;
        let t156 = piecewise3(t65, 0.0, 3.0 / 20.0 * t6 * t147 * t95 + t154);
        let tvrho0 = t64 + t99 + t7 * (t140 + t156);
        vrho[ip * 2] += tvrho0;
        let t160 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t102);
        let t163 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t160);
        let t164 = t163 * t30;
        let t169 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t164 * t60 + t116);
        let t171 = piecewise5(t15, 0.0, t11, 0.0, t8 - t141);
        let t174 = piecewise3(t70, 0.0, 5.0 / 3.0 * t72 * t171);
        let t175 = t174 * t30;
        let t179 = t86 * t86;
        let t180 = 1.0 / t179;
        let t181 = t180 * param_a;
        let t183 = 1.0 / t77 / t88;
        let t187 = piecewise3(t83, -t37 * t76 * t183 / 9.0, 0.0);
        let t188 = f64::sinh(t85);
        let t189 = t187 * t188;
        let t191 = t88 * rho1;
        let t193 = 1.0 / t89 / t191;
        let t197 = -t181 * t189 - 5.0 / 27.0 * t52 * sigma2 * t193;
        let t202 = piecewise3(t65, 0.0, 3.0 / 20.0 * t6 * t175 * t95 + t154 + 3.0 / 20.0 * t6 * t75 * t197);
        let tvrho1 = t64 + t99 + t7 * (t169 + t202);
        vrho[ip * 2 + 1] += tvrho1;
        let t205 = 1.0 / t38;
        let t209 = piecewise3(t45, t37 * t205 * t41 / 24.0, 0.0);
        let t210 = t209 * t126;
        let t214 = -t119 * t210 + 5.0 / 72.0 * t52 * t56;
        let t218 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t214);
        let tvsigma0 = t7 * t218;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t219 = 1.0 / t76;
        let t223 = piecewise3(t83, t37 * t219 * t79 / 24.0, 0.0);
        let t224 = t223 * t188;
        let t228 = -t181 * t224 + 5.0 / 72.0 * t52 * t91;
        let t232 = piecewise3(t65, 0.0, 3.0 / 20.0 * t6 * t75 * t228);
        let tvsigma2 = t7 * t232;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
