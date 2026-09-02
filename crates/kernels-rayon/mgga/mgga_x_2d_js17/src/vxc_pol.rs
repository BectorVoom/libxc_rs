//! MGGA_X_2D_JS17 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_2d_js17_vxc_pol(
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
        let t3 = rmath::sqrt(M_PI);
        let t4 = 1.0 / t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = rmath::sqrt(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = rmath::sqrt(t18);
        let t23 = t22 * t18;
        let t24 = piecewise3(t19, t21, t23);
        let t25 = t4 * t24;
        let t26 = M_SQRT2;
        let t27 = rmath::sqrt(t5);
        let t28 = t26 * t27;
        let t29 = rho0 * rho0;
        let t30 = t29 * rho0;
        let t31 = 1.0 / t30;
        let t32 = sigma0 * t31;
        let t34 = sigma0 * sigma0;
        let t35 = t29 * t29;
        let t37 = 1.0 / t35 / t29;
        let t40 = 1.0 + 0.41252961249419273 * t32 + 0.0006302988192022548 * t34 * t37;
        let t41 = rmath::pow(t40, 1.0 / 15.0);
        let t44 = 1.0 / t29;
        let t48 = 1.0 / M_PI;
        let t51 = 1.0 + 0.02793851343876014 * t32 + (-0.0772 * tau0 * t44 - 11.596246802930645) * t48 / 4.0;
        let t52 = rmath::pow(t40, 1.0 / 5.0);
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t41 + 2.0 / 5.0 * t51 * t53;
        let t57 = t28 * t56;
        let t60 = piecewise3(t2, 0.0, -2.0 / 3.0 * t25 * t57);
        let t61 = rho1 <= dens_threshold;
        let t62 = -t15;
        let t64 = piecewise5(t13, t10, t9, t14, t62 * t6);
        let t65 = 1.0 + t64;
        let t66 = t65 <= zeta_threshold;
        let t67 = rmath::sqrt(t65);
        let t68 = t67 * t65;
        let t69 = piecewise3(t66, t21, t68);
        let t70 = t4 * t69;
        let t71 = rho1 * rho1;
        let t72 = t71 * rho1;
        let t73 = 1.0 / t72;
        let t74 = sigma2 * t73;
        let t76 = sigma2 * sigma2;
        let t77 = t71 * t71;
        let t79 = 1.0 / t77 / t71;
        let t82 = 1.0 + 0.41252961249419273 * t74 + 0.0006302988192022548 * t76 * t79;
        let t83 = rmath::pow(t82, 1.0 / 15.0);
        let t86 = 1.0 / t71;
        let t92 = 1.0 + 0.02793851343876014 * t74 + (-0.0772 * tau1 * t86 - 11.596246802930645) * t48 / 4.0;
        let t93 = rmath::pow(t82, 1.0 / 5.0);
        let t94 = 1.0 / t93;
        let t97 = 1.0 / t83 + 2.0 / 5.0 * t92 * t94;
        let t98 = t28 * t97;
        let t101 = piecewise3(t61, 0.0, -2.0 / 3.0 * t70 * t98);
        let tzk0 = t60 + t101;
        zk[ip] += tzk0;
        let t102 = t5 * t5;
        let t103 = 1.0 / t102;
        let t104 = t15 * t103;
        let t106 = piecewise5(t9, 0.0, t13, 0.0, t6 - t104);
        let t109 = piecewise3(t19, 0.0, 3.0 / 2.0 * t22 * t106);
        let t110 = t4 * t109;
        let t113 = 1.0 / t27;
        let t114 = t26 * t113;
        let t115 = t114 * t56;
        let t117 = t25 * t115 / 3.0;
        let t119 = 1.0 / t41 / t40;
        let t120 = 1.0 / t35;
        let t121 = sigma0 * t120;
        let t124 = 1.0 / t35 / t30;
        let t127 = -1.237588837482578 * t121 - 0.003781792915213529 * t34 * t124;
        let t133 = -0.08381554031628043 * t121 + 0.01228676160669432 * tau0 * t31;
        let t137 = 1.0 / t52 / t40;
        let t138 = t51 * t137;
        let t141 = -t119 * t127 / 15.0 + 2.0 / 5.0 * t133 * t53 - 2.0 / 25.0 * t138 * t127;
        let t142 = t28 * t141;
        let t146 = piecewise3(t2, 0.0, -2.0 / 3.0 * t110 * t57 - t117 - 2.0 / 3.0 * t25 * t142);
        let t147 = t62 * t103;
        let t149 = piecewise5(t13, 0.0, t9, 0.0, -t6 - t147);
        let t152 = piecewise3(t66, 0.0, 3.0 / 2.0 * t67 * t149);
        let t153 = t4 * t152;
        let t156 = t114 * t97;
        let t158 = t70 * t156 / 3.0;
        let t160 = piecewise3(t61, 0.0, -2.0 / 3.0 * t153 * t98 - t158);
        let tvrho0 = t60 + t101 + t5 * (t146 + t160);
        vrho[ip * 2] += tvrho0;
        let t164 = piecewise5(t9, 0.0, t13, 0.0, -t6 - t104);
        let t167 = piecewise3(t19, 0.0, 3.0 / 2.0 * t22 * t164);
        let t168 = t4 * t167;
        let t172 = piecewise3(t2, 0.0, -2.0 / 3.0 * t168 * t57 - t117);
        let t174 = piecewise5(t13, 0.0, t9, 0.0, t6 - t147);
        let t177 = piecewise3(t66, 0.0, 3.0 / 2.0 * t67 * t174);
        let t178 = t4 * t177;
        let t182 = 1.0 / t83 / t82;
        let t183 = 1.0 / t77;
        let t184 = sigma2 * t183;
        let t187 = 1.0 / t77 / t72;
        let t190 = -1.237588837482578 * t184 - 0.003781792915213529 * t76 * t187;
        let t196 = -0.08381554031628043 * t184 + 0.01228676160669432 * tau1 * t73;
        let t200 = 1.0 / t93 / t82;
        let t201 = t92 * t200;
        let t204 = -t182 * t190 / 15.0 + 2.0 / 5.0 * t196 * t94 - 2.0 / 25.0 * t201 * t190;
        let t205 = t28 * t204;
        let t209 = piecewise3(t61, 0.0, -2.0 / 3.0 * t178 * t98 - t158 - 2.0 / 3.0 * t70 * t205);
        let tvrho1 = t60 + t101 + t5 * (t172 + t209);
        vrho[ip * 2 + 1] += tvrho1;
        let t213 = sigma0 * t37;
        let t215 = 0.41252961249419273 * t31 + 0.0012605976384045096 * t213;
        let t222 = -t119 * t215 / 15.0 + 0.011175405375504056 * t31 * t53 - 2.0 / 25.0 * t138 * t215;
        let t223 = t28 * t222;
        let t226 = piecewise3(t2, 0.0, -2.0 / 3.0 * t25 * t223);
        let tvsigma0 = t5 * t226;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t228 = sigma2 * t79;
        let t230 = 0.41252961249419273 * t73 + 0.0012605976384045096 * t228;
        let t237 = -t182 * t230 / 15.0 + 0.011175405375504056 * t73 * t94 - 2.0 / 25.0 * t201 * t230;
        let t238 = t28 * t237;
        let t241 = piecewise3(t61, 0.0, -2.0 / 3.0 * t70 * t238);
        let tvsigma2 = t5 * t241;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t242 = t24 * t26;
        let t244 = t27 * t44 * t53;
        let t247 = piecewise3(t2, 0.0, 0.0009242750552041906 * t242 * t244);
        let tvtau0 = t5 * t247;
        vtau[ip * 2] += tvtau0;
        let t248 = t69 * t26;
        let t250 = t27 * t86 * t94;
        let t253 = piecewise3(t61, 0.0, 0.0009242750552041906 * t248 * t250);
        let tvtau1 = t5 * t253;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
