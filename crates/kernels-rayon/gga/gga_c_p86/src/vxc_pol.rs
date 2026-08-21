//! GGA_C_P86 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_p86_vxc_pol(
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
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = M_CBRT3;
        let t2 = 1.0 / M_PI;
        let t3 = pow_1_3(t2);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t9 = 1.0 / t8;
        let t10 = t6 * t9;
        let t11 = t4 * t10;
        let t12 = t11 / 4.0;
        let t13 = 1.0 <= t12;
        let t14 = rmath::sqrt(t11);
        let t17 = 1.0 + 0.52645 * t14 + 0.08335 * t11;
        let t20 = rmath::ln(t12);
        let t23 = t4 * t10 * t20;
        let t27 = piecewise3(t13, -0.1423 / t17, 0.0311 * t20 - 0.048 + 0.0005 * t23 - 0.0029 * t11);
        let t30 = 1.0 + 0.69905 * t14 + 0.065275 * t11;
        let t37 = piecewise3(t13, -0.0843 / t30, 0.01555 * t20 - 0.0269 + 0.000175 * t23 - 0.0012 * t11);
        let t38 = t37 - t27;
        let t39 = rho0 - rho1;
        let t40 = 1.0 / t7;
        let t41 = t39 * t40;
        let t42 = 1.0 + t41;
        let t43 = t42 <= zeta_threshold;
        let t44 = pow_1_3(zeta_threshold);
        let t45 = t44 * zeta_threshold;
        let t46 = pow_1_3(t42);
        let t47 = t46 * t42;
        let t48 = piecewise3(t43, t45, t47);
        let t49 = 1.0 - t41;
        let t50 = t49 <= zeta_threshold;
        let t51 = pow_1_3(t49);
        let t52 = t51 * t49;
        let t53 = piecewise3(t50, t45, t52);
        let t54 = t48 + t53 - 2.0;
        let t56 = M_CBRT2;
        let t59 = 1.0 / (2.0 * t56 - 2.0);
        let t60 = t38 * t54 * t59;
        let t62 = sigma0 + 2.0 * sigma1 + sigma2;
        let t63 = t7 * t7;
        let t65 = 1.0 / t8 / t63;
        let t66 = t62 * t65;
        let t67 = param_aa + param_bb;
        let t68 = param_ftilde * t67;
        let t69 = param_malpha * t1;
        let t70 = t3 * t6;
        let t71 = t70 * t9;
        let t74 = t1 * t1;
        let t75 = param_mbeta * t74;
        let t76 = t3 * t3;
        let t77 = t76 * t5;
        let t78 = t8 * t8;
        let t79 = 1.0 / t78;
        let t80 = t77 * t79;
        let t83 = param_bb + t69 * t71 / 4.0 + t75 * t80 / 4.0;
        let t84 = param_mgamma * t1;
        let t87 = param_mdelta * t74;
        let t92 = 1.0 + t84 * t71 / 4.0 + t87 * t80 / 4.0 + 2387.32414637843 * param_mbeta * t40;
        let t93 = 1.0 / t92;
        let t95 = t83 * t93 + param_aa;
        let t96 = 1.0 / t95;
        let t97 = rmath::sqrt(t62);
        let t98 = t96 * t97;
        let t99 = rmath::pow(t7, 1.0 / 6.0);
        let t101 = 1.0 / t99 / t7;
        let t104 = rmath::exp(-t68 * t98 * t101);
        let t105 = t66 * t104;
        let t106 = t44 * t44;
        let t107 = t106 * zeta_threshold;
        let t108 = t46 * t46;
        let t109 = t108 * t42;
        let t110 = piecewise3(t43, t107, t109);
        let t111 = t51 * t51;
        let t112 = t111 * t49;
        let t113 = piecewise3(t50, t107, t112);
        let t114 = t110 + t113;
        let t115 = rmath::sqrt(t114);
        let t116 = 1.0 / t115;
        let t117 = t95 * t116;
        let t118 = M_SQRT2;
        let t119 = t117 * t118;
        let t120 = t105 * t119;
        let tzk0 = t27 + t60 + t120;
        zk[ip] += tzk0;
        let t121 = t17 * t17;
        let t122 = 1.0 / t121;
        let t124 = 1.0 / t14 * t1;
        let t126 = 1.0 / t8 / t7;
        let t127 = t70 * t126;
        let t128 = t124 * t127;
        let t130 = t6 * t126;
        let t131 = t4 * t130;
        let t133 = -0.08774166666666666 * t128 - 0.027783333333333333 * t131;
        let t138 = t4 * t130 * t20;
        let t142 = piecewise3(t13, 0.1423 * t122 * t133, -0.010366666666666666 * t40 - 0.00016666666666666666 * t138 + 0.0008 * t131);
        let t143 = t30 * t30;
        let t144 = 1.0 / t143;
        let t147 = -0.11650833333333334 * t128 - 0.021758333333333334 * t131;
        let t154 = piecewise3(t13, 0.0843 * t144 * t147, -0.005183333333333333 * t40 - 5.833333333333333e-05 * t138 + 0.00034166666666666666 * t131);
        let t155 = t154 - t142;
        let t157 = t155 * t54 * t59;
        let t158 = 1.0 / t63;
        let t159 = t39 * t158;
        let t160 = t40 - t159;
        let t163 = piecewise3(t43, 0.0, 4.0 / 3.0 * t46 * t160);
        let t164 = -t160;
        let t167 = piecewise3(t50, 0.0, 4.0 / 3.0 * t51 * t164);
        let t168 = t163 + t167;
        let t170 = t38 * t168 * t59;
        let t171 = t63 * t7;
        let t173 = 1.0 / t8 / t171;
        let t174 = t62 * t173;
        let t175 = t174 * t104;
        let t176 = t175 * t119;
        let t177 = 7.0 / 3.0 * t176;
        let t178 = t95 * t95;
        let t179 = 1.0 / t178;
        let t180 = t68 * t179;
        let t181 = t97 * t101;
        let t186 = t77 / t78 / t7;
        let t189 = -t69 * t127 / 12.0 - t75 * t186 / 6.0;
        let t191 = t92 * t92;
        let t192 = 1.0 / t191;
        let t193 = t83 * t192;
        let t200 = -t84 * t127 / 12.0 - t87 * t186 / 6.0 - 2387.32414637843 * param_mbeta * t158;
        let t202 = t189 * t93 - t193 * t200;
        let t206 = 1.0 / t99 / t63;
        let t210 = t180 * t181 * t202 + 7.0 / 6.0 * t68 * t98 * t206;
        let t211 = t66 * t210;
        let t212 = t104 * t95;
        let t213 = t116 * t118;
        let t214 = t212 * t213;
        let t215 = t211 * t214;
        let t216 = t202 * t116;
        let t217 = t216 * t118;
        let t218 = t105 * t217;
        let t220 = 1.0 / t115 / t114;
        let t221 = t95 * t220;
        let t224 = piecewise3(t43, 0.0, 5.0 / 3.0 * t108 * t160);
        let t227 = piecewise3(t50, 0.0, 5.0 / 3.0 * t111 * t164);
        let t228 = t224 + t227;
        let t229 = t118 * t228;
        let t230 = t221 * t229;
        let t231 = t105 * t230;
        let t232 = t231 / 2.0;
        let tvrho0 = t27 + t60 + t120 + t7 * (t142 + t157 + t170 - t177 + t215 + t218 - t232);
        vrho[ip * 2] += tvrho0;
        let t235 = -t40 - t159;
        let t238 = piecewise3(t43, 0.0, 4.0 / 3.0 * t46 * t235);
        let t239 = -t235;
        let t242 = piecewise3(t50, 0.0, 4.0 / 3.0 * t51 * t239);
        let t243 = t238 + t242;
        let t245 = t38 * t243 * t59;
        let t248 = piecewise3(t43, 0.0, 5.0 / 3.0 * t108 * t235);
        let t251 = piecewise3(t50, 0.0, 5.0 / 3.0 * t111 * t239);
        let t252 = t248 + t251;
        let t253 = t118 * t252;
        let t254 = t221 * t253;
        let t255 = t105 * t254;
        let t256 = t255 / 2.0;
        let tvrho1 = t27 + t60 + t120 + t7 * (t142 + t157 + t245 - t177 + t215 + t218 - t256);
        vrho[ip * 2 + 1] += tvrho1;
        let t259 = t65 * t104;
        let t260 = t259 * t119;
        let t261 = rmath::sqrt(t7);
        let t263 = 1.0 / t261 / t171;
        let t264 = t97 * t263;
        let t267 = t67 * t104 * t213;
        let t268 = t264 * param_ftilde * t267;
        let t269 = t268 / 2.0;
        let tvsigma0 = t7 * (t260 - t269);
        vsigma[ip * 3] += tvsigma0;
        let t271 = 2.0 * t260;
        let tvsigma1 = t7 * (t271 - t268);
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = tvsigma0;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
