//! GGA_X_BEEFVDW exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_beefvdw_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = t18 + 1.0;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t39 = sigma0 * t38;
        let t42 = 4.0 + t33 * t39 / 24.0;
        let t43 = 1.0 / t42;
        let t45 = t33 * t39 * t43;
        let t47 = t45 / 12.0 - 1.0;
        let t48 = t47 * t47;
        let t49 = t48 * t48;
        let t51 = t48 * t47;
        let t54 = t49 * t49;
        let t55 = t54 * t54;
        let t57 = t49 * t47;
        let t58 = t54 * t57;
        let t60 = t49 * t48;
        let t61 = t54 * t60;
        let t63 = t54 * t51;
        let t65 = t54 * t49;
        let t68 = t49 * t51;
        let t71 = t55 * t49;
        let t73 = t55 * t57;
        let t75 = t55 * t60;
        let t77 = t55 * t54;
        let t79 = -0.6945973517763898 * t49 + 0.527556201155898 * t51 - 0.38916037779196816 * t48 - 168370.8413901412 * t55 - 2810.240180568463 * t58 + 70504.54186903402 * t61 + 2274.8997850816486 * t63 - 20148.24517562505 * t65 - 442.33229018433804 * t54 + 86.00573049927964 * t68 + 30.54203495931585 * t60 - 323524.0313604933 * t71 + 180782.00670879145 * t73 + 255894.79526235335 * t75 - 132044.6618218215 * t77;
        let t80 = t55 * t68;
        let t82 = t54 * t47;
        let t83 = t55 * t82;
        let t85 = t55 * t65;
        let t87 = t54 * t48;
        let t88 = t55 * t87;
        let t90 = t55 * t63;
        let t94 = t55 * t51;
        let t96 = t55 * t47;
        let t98 = t55 * t48;
        let t104 = t54 * t68;
        let t106 = 1.1313514630621233 - 161142.1539984628 * t80 + 90365.6111085228 * t83 - 5427.777462637186 * t85 + 40074.93585443239 * t88 - 29150.193011493262 * t90 + 4135.586188014654 * t55 * t58 - 129814.81812794984 * t94 + 56174.00797937267 * t96 + 279670.48856303055 * t98 + 3783.53964072524 * t87 - 7.2975787893717134 * t57 - 617.547861045286 * t82 + 0.037534251004296526 * t45 - 10276.426607863825 * t104;
        let t107 = t79 + t106;
        let t111 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t107);
        let t112 = rho1 <= dens_threshold;
        let t113 = -t16;
        let t115 = piecewise5(t14, t11, t10, t15, t113 * t7);
        let t116 = t115 + 1.0;
        let t117 = t116 <= zeta_threshold;
        let t118 = pow_1_3(t116);
        let t120 = piecewise3(t117, t22, t118 * t116);
        let t121 = t120 * t26;
        let t122 = rho1 * rho1;
        let t123 = pow_1_3(rho1);
        let t124 = t123 * t123;
        let t126 = 1.0 / t124 / t122;
        let t127 = sigma2 * t126;
        let t130 = 4.0 + t33 * t127 / 24.0;
        let t131 = 1.0 / t130;
        let t133 = t33 * t127 * t131;
        let t135 = t133 / 12.0 - 1.0;
        let t136 = t135 * t135;
        let t137 = t136 * t135;
        let t138 = t136 * t136;
        let t139 = t138 * t137;
        let t140 = t138 * t138;
        let t141 = t140 * t139;
        let t143 = t140 * t138;
        let t145 = t138 * t135;
        let t146 = t140 * t145;
        let t148 = t140 * t137;
        let t152 = t140 * t135;
        let t155 = t140 * t140;
        let t156 = t155 * t148;
        let t160 = t138 * t136;
        let t163 = t155 * t140;
        let t165 = t155 * t152;
        let t167 = -10276.426607863825 * t141 - 20148.24517562505 * t143 - 2810.240180568463 * t146 + 2274.8997850816486 * t148 + 86.00573049927964 * t139 - 442.33229018433804 * t140 - 617.547861045286 * t152 + 0.527556201155898 * t137 - 29150.193011493262 * t156 - 0.38916037779196816 * t136 - 7.2975787893717134 * t145 + 30.54203495931585 * t160 - 0.6945973517763898 * t138 - 132044.6618218215 * t163 + 90365.6111085228 * t165;
        let t169 = t155 * t135;
        let t171 = t140 * t160;
        let t175 = t155 * t143;
        let t177 = t155 * t139;
        let t179 = t155 * t145;
        let t181 = t155 * t160;
        let t183 = t155 * t138;
        let t185 = t155 * t136;
        let t187 = t155 * t137;
        let t189 = t140 * t136;
        let t192 = t155 * t189;
        let t194 = 1.1313514630621233 - 168370.8413901412 * t155 + 56174.00797937267 * t169 + 70504.54186903402 * t171 + 4135.586188014654 * t155 * t146 - 5427.777462637186 * t175 - 161142.1539984628 * t177 + 180782.00670879145 * t179 + 255894.79526235335 * t181 - 323524.0313604933 * t183 + 279670.48856303055 * t185 - 129814.81812794984 * t187 + 3783.53964072524 * t189 + 0.037534251004296526 * t133 + 40074.93585443239 * t192;
        let t195 = t167 + t194;
        let t199 = piecewise3(t112, 0.0, -3.0 / 8.0 * t5 * t121 * t195);
        let tzk0 = t111 + t199;
        zk[ip] += tzk0;
    }
}
