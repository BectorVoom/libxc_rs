//! GGA_X_SG4 kxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sg4_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let t11 = 1.0 + t10;
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
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = 1.0 - 0.3123398257303946694e-2 * t34;
        let t37 = t20 * t20;
        let t38 = t21 * t21;
        let t39 = t38 * t21;
        let t41 = 1.0 / t22 / t39;
        let t42 = t37 * t41;
        let t43 = sigma[ip] * sigma[ip];
        let t44 = t43 * t43;
        let t45 = t44 * sigma[ip];
        let t47 = t29 * t29;
        let t48 = t47 * rho[ip];
        let t49 = t47 * t47;
        let t50 = t49 * t48;
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 - 0.14268491327672029207e-10 * t42 * t45 * t26 * t52;
        let t57 = 1.0 / t56;
        let t61 = 1.0 + 0.37270642201834862386e-1 * t34;
        let t64 = 0.1804e1 - 0.56028717948717948718e0 * t36 * t57 - 0.24371282051282051282e0 / t61;
        let t68 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t64);
        let tzk0 = 2.0 * t68;
        zk[ip] += tzk0;
        let t70 = t17 / t30;
        let t74 = t25 * sigma[ip];
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t79 = t27 * t77 * t57;
        let t82 = t56 * t56;
        let t83 = 1.0 / t82;
        let t85 = t36 * t83 * t37;
        let t86 = t41 * t45;
        let t87 = t47 * t29;
        let t88 = t49 * t87;
        let t91 = t26 / t18 / t88;
        let t95 = t61 * t61;
        let t97 = 1.0 / t95 * t20;
        let t98 = t97 * t24;
        let t102 = -0.46666666666666666667e-2 * t74 * t79 + 0.10659270348691522892e-9 * t85 * t86 * t91 - 0.24222222222222222223e-1 * t98 * t28 * t77;
        let t107 = piecewise3(t2, 0.0, -t6 * t70 * t64 / 8.0 - 3.0 / 8.0 * t6 * t19 * t102);
        let tvrho0 = 2.0 * rho[ip] * t107 + 2.0 * t68;
        vrho[ip] += tvrho0;
        let t114 = t41 * t44;
        let t115 = t26 * t52;
        let t119 = t24 * t27;
        let t123 = 0.175e-2 * t25 * t27 * t32 * t57 - 0.39972263807593210847e-10 * t85 * t114 * t115 + 0.90833333333333333335e-2 * t97 * t119 * t32;
        let t127 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t123);
        let tvsigma0 = 2.0 * rho[ip] * t127;
        vsigma[ip] += tvsigma0;
        let t132 = t17 / t30 / rho[ip];
        let t140 = 1.0 / t30 / t47;
        let t142 = t27 * t140 * t57;
        let t145 = t44 * t43;
        let t146 = t49 * t49;
        let t148 = 1.0 / t146 / t29;
        let t153 = 1.0 / t82 / t56;
        let t155 = t36 * t153 * t20;
        let t156 = t38 * t38;
        let t159 = 1.0 / t23 / t156 / t38;
        let t160 = t44 * t44;
        let t161 = t160 * t43;
        let t162 = t159 * t161;
        let t163 = t49 * t47;
        let t167 = t27 / t30 / t146 / t163;
        let t171 = t47 * t75;
        let t172 = t49 * t171;
        let t175 = t26 / t18 / t172;
        let t181 = 1.0 / t95 / t61 * t37;
        let t183 = 1.0 / t22 / t21;
        let t184 = t181 * t183;
        let t185 = t43 * t26;
        let t187 = 1.0 / t18 / t171;
        let t194 = 0.17111111111111111111e-1 * t74 * t142 + 0.22456177547295639295e-14 * t145 * t148 * t83 - 0.24334673044738656188e-18 * t155 * t162 * t167 - 0.15278287499791182812e-8 * t85 * t86 * t175 - 0.96296296296296296303e-2 * t184 * t185 * t187 + 0.88814814814814814818e-1 * t98 * t28 * t140;
        let t199 = piecewise3(t2, 0.0, t6 * t132 * t64 / 12.0 - t6 * t70 * t102 / 4.0 - 3.0 / 8.0 * t6 * t19 * t194);
        let tv2rho20 = 2.0 * rho[ip] * t199 + 4.0 * t107;
        v2rho2[ip] += tv2rho20;
        let t207 = t146 * rho[ip];
        let t209 = 1.0 / t207 * t83;
        let t212 = t160 * sigma[ip];
        let t213 = t159 * t212;
        let t214 = t49 * t75;
        let t218 = t27 / t30 / t146 / t214;
        let t226 = 1.0 / t18 / t87;
        let t234 = -0.46666666666666666667e-2 * t25 * t79 - 0.84210665802358647355e-15 * t209 * t45 + 0.91255023917769960709e-19 * t155 * t213 * t218 + 0.53296351743457614463e-9 * t85 * t114 * t91 + 0.36111111111111111113e-2 * t184 * t26 * t226 * sigma[ip] - 0.24222222222222222223e-1 * t97 * t119 * t77;
        let t239 = piecewise3(t2, 0.0, -t6 * t70 * t123 / 8.0 - 3.0 / 8.0 * t6 * t19 * t234);
        let tv2rhosigma0 = 2.0 * rho[ip] * t239 + 2.0 * t127;
        v2rhosigma[ip] += tv2rhosigma0;
        let t243 = 1.0 / t146 * t83;
        let t246 = t159 * t160;
        let t247 = t49 * t29;
        let t251 = t27 / t30 / t146 / t247;
        let t255 = t43 * sigma[ip];
        let t256 = t41 * t255;
        let t260 = t183 * t26;
        let t266 = 0.31578999675884492758e-15 * t243 * t44 - 0.34220633969163735268e-19 * t155 * t246 * t251 - 0.15988905523037284339e-9 * t85 * t256 * t115 - 0.13541666666666666667e-2 * t181 * t260 / t18 / t48;
        let t270 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t266);
        let tv2sigma20 = 2.0 * rho[ip] * t270;
        v2sigma2[ip] += tv2sigma20;
        let t273 = t17 * t32;
        let t284 = 1.0 / t30 / t48;
        let t286 = t27 * t284 * t57;
        let t290 = 1.0 / t146 / t75;
        let t294 = t160 * t255;
        let t295 = t146 * t146;
        let t297 = 1.0 / t18 / t295;
        let t300 = t42 * t26;
        let t306 = t37 / t22 / t156 / t39;
        let t307 = t306 * t294;
        let t309 = t26 * t297 * t153;
        let t312 = t82 * t82;
        let t313 = 1.0 / t312;
        let t314 = t36 * t313;
        let t315 = t44 * t255;
        let t316 = t160 * t315;
        let t317 = t295 * t214;
        let t318 = 1.0 / t317;
        let t322 = t146 * t50;
        let t325 = t27 / t30 / t322;
        let t331 = t26 / t18 / t146;
        let t335 = t95 * t95;
        let t336 = 1.0 / t335;
        let t337 = t336 * t255;
        let t338 = 1.0 / t214;
        let t342 = 1.0 / t18 / t49;
        let t349 = -0.79851851851851851851e-1 * t74 * t286 - 0.60631679377698226095e-13 * t145 * t290 * t83 - 0.85444206556333645005e-24 * t294 * t297 * t153 * t300 - 0.40537000202665624096e-20 * t307 * t309 + 0.1900410208391527399e-36 * t314 * t316 * t318 + 0.10463909409237622161e-16 * t155 * t162 * t325 + 0.23426707499679813645e-7 * t85 * t86 * t331 - 0.3537107035762798465e-3 * t337 * t338 + 0.10592592592592592593e0 * t184 * t185 * t342 - 0.41446913580246913582e0 * t98 * t28 * t284;
        let t354 = piecewise3(t2, 0.0, -5.0 / 36.0 * t6 * t273 * t64 + t6 * t132 * t102 / 4.0 - 3.0 / 8.0 * t6 * t70 * t194 - 3.0 / 8.0 * t6 * t19 * t349);
        let tv3rho30 = 2.0 * rho[ip] * t354 + 6.0 * t199;
        v3rho3[ip] += tv3rho30;
        let t366 = t148 * t83;
        let t371 = 1.0 / t18 / t146 / t172;
        let t372 = t371 * t153;
        let t376 = t306 * t161;
        let t378 = t26 * t371 * t153;
        let t381 = t160 * t145;
        let t382 = t295 * t247;
        let t383 = 1.0 / t382;
        let t394 = t336 / t247;
        let t404 = 0.17111111111111111111e-1 * t25 * t142 + 0.21052666450589661838e-13 * t366 * t45 + 0.32041577458625116876e-24 * t372 * t161 * t300 + 0.15201375075999609037e-20 * t376 * t378 - 0.71265382814682277465e-37 * t314 * t381 * t383 - 0.37414559806285683891e-17 * t155 * t213 * t167 - 0.76391437498955914063e-8 * t85 * t114 * t175 + 0.13264151384110494244e-3 * t394 * t43 - 0.32500000000000000002e-1 * t184 * t26 * t187 * sigma[ip] + 0.88814814814814814818e-1 * t97 * t119 * t140;
        let t409 = piecewise3(t2, 0.0, t6 * t132 * t123 / 12.0 - t6 * t70 * t234 / 4.0 - 3.0 / 8.0 * t6 * t19 * t404);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t409 + 4.0 * t239;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t417 = t146 * t88;
        let t419 = 1.0 / t18 / t417;
        let t420 = t419 * t153;
        let t424 = t306 * t212;
        let t429 = t160 * t45;
        let t430 = t49 * rho[ip];
        let t432 = 1.0 / t295 / t430;
        let t443 = t336 / t430;
        let t449 = -0.67368532641886917884e-14 * t209 * t44 - 0.12015591546984418829e-24 * t420 * t212 * t300 - 0.57005156534998533891e-21 * t424 * t26 * t419 * t153 + 0.26724518555505854051e-37 * t314 * t429 * t432 + 0.127757033484877945e-17 * t155 * t246 * t218 + 0.21318540697383045785e-8 * t85 * t256 * t91 - 0.49740567690414353412e-4 * t443 * sigma[ip] + 0.72222222222222222224e-2 * t181 * t260 * t226;
        let t454 = piecewise3(t2, 0.0, -t6 * t70 * t266 / 8.0 - 3.0 / 8.0 * t6 * t19 * t449);
        let tv3rhosigma20 = 2.0 * rho[ip] * t454 + 2.0 * t270;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t459 = 1.0 / t18 / t322 * t153;
        let t460 = t459 * t160;
        let t465 = t306 * t26;
        let t468 = t160 * t44;
        let t470 = 1.0 / t295 / t49;
        let t474 = t159 * t315;
        let t478 = t41 * t43;
        let t485 = 0.45058468301191570608e-25 * t460 * t300 + 0.18947399805530695655e-14 * t243 * t255 + 0.21376933700624450209e-21 * t465 * t460 - 0.10021694458314695269e-37 * t314 * t468 * t470 - 0.41064760762996482321e-18 * t155 * t474 * t251 - 0.47966716569111853017e-9 * t85 * t478 * t115 + 0.18652712883905382528e-4 * t336 / t49;
        let t489 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t485);
        let tv3sigma30 = 2.0 * rho[ip] * t489;
        v3sigma3[ip] += tv3sigma30;
    }
}
