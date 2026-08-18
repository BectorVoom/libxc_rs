//! HYB_MGGA_X_DLDF vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_dldf_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = pow_1_3(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = pow_1_3(t17);
        let t23 = piecewise3(t18, t20, t21 * t17);
        let t24 = t3 * t23;
        let t25 = pow_1_3(t4);
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = rho0 * rho0;
        let t33 = pow_1_3(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / t32;
        let t40 = 4.8827323 + 0.0146297 * t31 * sigma0 * t36;
        let t43 = 5.8827323 - 23.84107471346329 / t40;
        let t44 = t25 * t43;
        let t45 = t26 * t26;
        let t47 = 3.0 / 10.0 * t45 * t29;
        let t49 = 1.0 / t34 / rho0;
        let t50 = tau0 * t49;
        let t51 = t47 - t50;
        let t52 = t47 + t50;
        let t53 = 1.0 / t52;
        let t56 = t51 * t51;
        let t57 = t52 * t52;
        let t58 = 1.0 / t57;
        let t61 = t56 * t51;
        let t62 = t57 * t52;
        let t63 = 1.0 / t62;
        let t66 = t56 * t56;
        let t67 = t57 * t57;
        let t68 = 1.0 / t67;
        let t71 = 1.0 - 0.1637571 * t51 * t53 - 0.1880028 * t56 * t58 - 0.4490609 * t61 * t63 - 0.0082359 * t66 * t68;
        let t72 = t44 * t71;
        let t75 = piecewise3(t2, 0.0, -0.09872727257880975 * t24 * t72);
        let t76 = rho1 <= dens_threshold;
        let t77 = -t14;
        let t79 = piecewise5(t12, t9, t8, t13, t77 * t5);
        let t80 = 1.0 + t79;
        let t81 = t80 <= zeta_threshold;
        let t82 = pow_1_3(t80);
        let t84 = piecewise3(t81, t20, t82 * t80);
        let t85 = t3 * t84;
        let t86 = rho1 * rho1;
        let t87 = pow_1_3(rho1);
        let t88 = t87 * t87;
        let t90 = 1.0 / t88 / t86;
        let t94 = 4.8827323 + 0.0146297 * t31 * sigma2 * t90;
        let t97 = 5.8827323 - 23.84107471346329 / t94;
        let t98 = t25 * t97;
        let t100 = 1.0 / t88 / rho1;
        let t101 = tau1 * t100;
        let t102 = t47 - t101;
        let t103 = t47 + t101;
        let t104 = 1.0 / t103;
        let t107 = t102 * t102;
        let t108 = t103 * t103;
        let t109 = 1.0 / t108;
        let t112 = t107 * t102;
        let t113 = t108 * t103;
        let t114 = 1.0 / t113;
        let t117 = t107 * t107;
        let t118 = t108 * t108;
        let t119 = 1.0 / t118;
        let t122 = 1.0 - 0.1637571 * t102 * t104 - 0.1880028 * t107 * t109 - 0.4490609 * t112 * t114 - 0.0082359 * t117 * t119;
        let t123 = t98 * t122;
        let t126 = piecewise3(t76, 0.0, -0.09872727257880975 * t85 * t123);
        let tzk0 = t75 + t126;
        zk[ip] += tzk0;
        let t127 = t4 * t4;
        let t128 = 1.0 / t127;
        let t129 = t14 * t128;
        let t131 = piecewise5(t8, 0.0, t12, 0.0, t5 - t129);
        let t134 = piecewise3(t18, 0.0, 4.0 / 3.0 * t21 * t131);
        let t135 = t3 * t134;
        let t138 = t25 * t25;
        let t139 = 1.0 / t138;
        let t140 = t139 * t43;
        let t141 = t140 * t71;
        let t143 = 0.03290909085960325 * t24 * t141;
        let t144 = t40 * t40;
        let t145 = 1.0 / t144;
        let t146 = t25 * t145;
        let t147 = t24 * t146;
        let t148 = t32 * rho0;
        let t150 = 1.0 / t34 / t148;
        let t151 = sigma0 * t150;
        let t153 = t31 * t151 * t71;
        let t156 = tau0 * t36;
        let t159 = t51 * t58;
        let t162 = t56 * t63;
        let t165 = t61 * t68;
        let t169 = 1.0 / t67 / t52;
        let t170 = t66 * t169;
        let t173 = -0.2729285 * t156 * t53 - 0.8996045 * t159 * t156 - 2.8719805 * t162 * t156 - 2.3002105 * t165 * t156 - 0.054906 * t170 * t156;
        let t174 = t44 * t173;
        let t178 = piecewise3(t2, 0.0, -0.09872727257880975 * t135 * t72 - t143 + 0.09182630750283849 * t147 * t153 - 0.09872727257880975 * t24 * t174);
        let t179 = t77 * t128;
        let t181 = piecewise5(t12, 0.0, t8, 0.0, -t5 - t179);
        let t184 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t181);
        let t185 = t3 * t184;
        let t188 = t139 * t97;
        let t189 = t188 * t122;
        let t191 = 0.03290909085960325 * t85 * t189;
        let t193 = piecewise3(t76, 0.0, -0.09872727257880975 * t185 * t123 - t191);
        let tvrho0 = t75 + t126 + t4 * (t178 + t193);
        vrho[ip * 2] += tvrho0;
        let t197 = piecewise5(t8, 0.0, t12, 0.0, -t5 - t129);
        let t200 = piecewise3(t18, 0.0, 4.0 / 3.0 * t21 * t197);
        let t201 = t3 * t200;
        let t205 = piecewise3(t2, 0.0, -0.09872727257880975 * t201 * t72 - t143);
        let t207 = piecewise5(t12, 0.0, t8, 0.0, t5 - t179);
        let t210 = piecewise3(t81, 0.0, 4.0 / 3.0 * t82 * t207);
        let t211 = t3 * t210;
        let t214 = t94 * t94;
        let t215 = 1.0 / t214;
        let t216 = t25 * t215;
        let t217 = t85 * t216;
        let t218 = t86 * rho1;
        let t220 = 1.0 / t88 / t218;
        let t221 = sigma2 * t220;
        let t223 = t31 * t221 * t122;
        let t226 = tau1 * t90;
        let t229 = t102 * t109;
        let t232 = t107 * t114;
        let t235 = t112 * t119;
        let t239 = 1.0 / t118 / t103;
        let t240 = t117 * t239;
        let t243 = -0.2729285 * t226 * t104 - 0.8996045 * t229 * t226 - 2.8719805 * t232 * t226 - 2.3002105 * t235 * t226 - 0.054906 * t240 * t226;
        let t244 = t98 * t243;
        let t248 = piecewise3(t76, 0.0, -0.09872727257880975 * t211 * t123 - t191 + 0.09182630750283849 * t217 * t223 - 0.09872727257880975 * t85 * t244);
        let tvrho1 = t75 + t126 + t4 * (t205 + t248);
        vrho[ip * 2 + 1] += tvrho1;
        let t252 = t31 * t36 * t71;
        let t255 = piecewise3(t2, 0.0, -0.03443486531356443 * t147 * t252);
        let tvsigma0 = t4 * t255;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t257 = t31 * t90 * t122;
        let t260 = piecewise3(t76, 0.0, -0.03443486531356443 * t217 * t257);
        let tvsigma2 = t4 * t260;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t271 = 0.1637571 * t49 * t53 + 0.5397627 * t159 * t49 + 1.7231883 * t162 * t49 + 1.3801263 * t165 * t49 + 0.0329436 * t170 * t49;
        let t272 = t44 * t271;
        let t275 = piecewise3(t2, 0.0, -0.09872727257880975 * t24 * t272);
        let tvtau0 = t4 * t275;
        vtau[ip * 2] += tvtau0;
        let t286 = 0.1637571 * t100 * t104 + 0.5397627 * t229 * t100 + 1.7231883 * t232 * t100 + 1.3801263 * t235 * t100 + 0.0329436 * t240 * t100;
        let t287 = t98 * t286;
        let t290 = piecewise3(t76, 0.0, -0.09872727257880975 * t85 * t287);
        let tvtau1 = t4 * t290;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
