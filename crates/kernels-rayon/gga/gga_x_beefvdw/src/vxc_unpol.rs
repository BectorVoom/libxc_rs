//! GGA_X_BEEFVDW vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_beefvdw_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = t10 + 1.0;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t38 = 4.0 + t25 * sigma[ip] * t28 * t32 / 24.0;
        let t39 = 1.0 / t38;
        let t40 = t28 * t32 * t39;
        let t41 = t26 * t40;
        let t43 = t41 / 12.0 - 1.0;
        let t44 = t43 * t43;
        let t45 = t44 * t44;
        let t46 = t45 * t45;
        let t47 = t46 * t45;
        let t48 = t46 * t46;
        let t49 = t48 * t47;
        let t51 = t45 * t43;
        let t52 = t46 * t51;
        let t55 = t44 * t43;
        let t56 = t46 * t55;
        let t57 = t48 * t56;
        let t59 = t46 * t44;
        let t60 = t48 * t59;
        let t62 = t46 * t43;
        let t63 = t48 * t62;
        let t65 = t45 * t55;
        let t66 = t48 * t65;
        let t68 = t48 * t46;
        let t70 = t45 * t44;
        let t71 = t48 * t70;
        let t78 = t48 * t44;
        let t81 = -5427.777462637186 * t49 + 4135.586188014654 * t48 * t52 - 29150.193011493262 * t57 + 40074.93585443239 * t60 + 90365.6111085228 * t63 - 161142.1539984628 * t66 - 132044.6618218215 * t68 + 255894.79526235335 * t71 - 0.6945973517763898 * t45 + 0.527556201155898 * t55 - 0.38916037779196816 * t44 + 86.00573049927964 * t65 + 30.54203495931585 * t70 + 279670.48856303055 * t78 + 0.037534251004296526 * t41;
        let t88 = t46 * t70;
        let t91 = t48 * t45;
        let t93 = t48 * t51;
        let t95 = t48 * t55;
        let t97 = t48 * t43;
        let t99 = t46 * t65;
        let t102 = 1.1313514630621233 - 7.2975787893717134 * t51 + 3783.53964072524 * t59 - 617.547861045286 * t62 - 442.33229018433804 * t46 - 20148.24517562505 * t47 + 2274.8997850816486 * t56 + 70504.54186903402 * t88 - 2810.240180568463 * t52 - 323524.0313604933 * t91 + 180782.00670879145 * t93 - 129814.81812794984 * t95 + 56174.00797937267 * t97 - 10276.426607863825 * t99 - 168370.8413901412 * t48;
        let t103 = t81 + t102;
        let t107 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t103);
        let tzk0 = 2.0 * t107;
        zk[ip] += tzk0;
        let t109 = t17 / t30;
        let t113 = t29 * rho[ip];
        let t115 = 1.0 / t30 / t113;
        let t117 = t28 * t115 * t39;
        let t118 = t26 * t117;
        let t120 = t20 * t20;
        let t122 = 1.0 / t22 / t21;
        let t123 = t120 * t122;
        let t124 = sigma[ip] * sigma[ip];
        let t125 = t123 * t124;
        let t126 = t29 * t29;
        let t127 = t126 * t29;
        let t129 = 1.0 / t18 / t127;
        let t131 = t38 * t38;
        let t132 = 1.0 / t131;
        let t133 = t27 * t129 * t132;
        let t134 = t125 * t133;
        let t136 = -2.0 / 9.0 * t118 + t134 / 54.0;
        let t167 = -6470480.6272098655 * t95 * t136 + 3796422.1408846206 * t91 * t136 - 2466481.544431047 * t78 * t136 + 954958.1356493353 * t48 * t136 + 5034068.79413455 * t97 * t136 - 154146.39911795736 * t88 * t136 - 2693933.462242259 * t99 * t136 - 36533.12234739002 * t47 * t136 + 987063.5861664761 * t52 * t136 + 25023.897635898134 * t59 * t136 - 241778.94210750057 * t56 * t136 - 3538.6583214747043 * t65 * t136 - 5557.930749407574 * t46 * t136 + 37835.3964072524 * t62 * t136 - 36.48789394685857 * t45 * t136;
        let t196 = 183.2522097558951 * t51 * t136 + 602.0401134949575 * t70 * t136 - 0.7783207555839363 * t43 * t136 + 1.582668603467694 * t44 * t136 - 2.7783894071055593 * t55 * t136 - 151977.7689538412 * t57 * t136 + 119931.99945242496 * t49 * t136 - 787055.2113103181 * t60 * t136 + 1041948.3322152421 * t63 * t136 + 2259140.27771307 * t68 * t136 - 3706269.5419646446 * t71 * t136 - 3169071.8837237163 * t66 * t136 + 5629685.495771773 * t93 * t136 - 0.10009133601145741 * t118 + 0.00834094466762145 * t134;
        let t197 = t167 + t196;
        let t202 = piecewise3(t2, 0.0, -t6 * t109 * t103 / 8.0 - 3.0 / 8.0 * t6 * t19 * t197);
        let tvrho0 = 2.0 * rho[ip] * t202 + 2.0 * t107;
        vrho[ip] += tvrho0;
        let t205 = t25 * t40;
        let t209 = t126 * rho[ip];
        let t213 = t27 / t18 / t209 * t132;
        let t214 = t123 * sigma[ip] * t213;
        let t216 = t205 / 12.0 - t214 / 144.0;
        let t217 = t47 * t216;
        let t219 = t52 * t216;
        let t221 = t59 * t216;
        let t223 = t56 * t216;
        let t225 = t65 * t216;
        let t227 = t46 * t216;
        let t229 = t62 * t216;
        let t231 = t45 * t216;
        let t233 = t51 * t216;
        let t235 = t70 * t216;
        let t237 = t43 * t216;
        let t239 = t44 * t216;
        let t241 = t55 * t216;
        let t243 = t57 * t216;
        let t245 = 0.037534251004296526 * t205 - 36533.12234739002 * t217 + 987063.5861664761 * t219 + 25023.897635898134 * t221 - 241778.94210750057 * t223 - 3538.6583214747043 * t225 - 5557.930749407574 * t227 + 37835.3964072524 * t229 - 36.48789394685857 * t231 + 183.2522097558951 * t233 + 602.0401134949575 * t235 - 0.7783207555839363 * t237 + 1.582668603467694 * t239 - 2.7783894071055593 * t241 - 151977.7689538412 * t243;
        let t248 = t60 * t216;
        let t250 = t63 * t216;
        let t252 = t68 * t216;
        let t254 = t71 * t216;
        let t256 = t66 * t216;
        let t258 = t93 * t216;
        let t260 = t95 * t216;
        let t262 = t91 * t216;
        let t264 = t78 * t216;
        let t266 = t48 * t216;
        let t268 = t97 * t216;
        let t270 = t88 * t216;
        let t272 = t99 * t216;
        let t275 = 119931.99945242496 * t49 * t216 - 787055.2113103181 * t248 + 1041948.3322152421 * t250 + 2259140.27771307 * t252 - 3706269.5419646446 * t254 - 3169071.8837237163 * t256 + 5629685.495771773 * t258 - 6470480.6272098655 * t260 + 3796422.1408846206 * t262 - 2466481.544431047 * t264 + 954958.1356493353 * t266 + 5034068.79413455 * t268 - 154146.39911795736 * t270 - 2693933.462242259 * t272 - 0.003127854250358044 * t214;
        let t276 = t245 + t275;
        let t280 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t276);
        let tvsigma0 = 2.0 * rho[ip] * t280;
        vsigma[ip] += tvsigma0;
    }
}
