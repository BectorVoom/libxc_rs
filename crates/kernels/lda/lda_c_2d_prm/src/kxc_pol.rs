//! LDA_C_2D_PRM kxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_prm.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI};
use libxc_kernel_math::powers::{pow_3_2};

/// LDA_C_2D_PRM kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_2d_prm_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = f64::sqrt(t1);
        let t4 = f64::sqrt(M_PI);
        let t6 = 3.9274 * t2 + t4 / 2.0;
        let t7 = 1.0 / t6;
        let t8 = t2 * t7;
        let t10 = 3.9274 * t8 - 1.0;
        let t11 = t2 * t10;
        let t12 = 2.0 + param_c;
        let t13 = f64::sqrt(t12);
        let t14 = 1.0 / t13;
        let t16 = 0.3544538369424879 * t11 * t14;
        let t17 = 1.0 / t12;
        let t18 = t10 * t17;
        let t20 = 0.3999583253029731 * t8 * t18;
        let t21 = t6 * t6;
        let t22 = 1.0 / t21;
        let t24 = 1.0/pow_3_2::<f64>(t12);
        let t26 = 0.17722691847124394 * t2 * t22 * t24;
        let t27 = 1.0 + param_c;
        let t28 = f64::sqrt(t27);
        let t29 = 1.0 / t28;
        let t31 = 0.7089076738849758 * t11 * t29;
        let t32 = 1.0 / t27;
        let t34 = 0.3999583253029731 * t8 * t32;
        let tzk0 = t16 + t20 + t26 + t31 + t34;
        zk[ip] += tzk0;
        let t35 = 1.0 / t2;
        let t36 = t35 * t10;
        let t37 = t36 * t14;
        let t39 = t35 * t7;
        let t42 = 1.9637 * t39 - 7.71223538 * t22;
        let t43 = t2 * t42;
        let t44 = t43 * t14;
        let t46 = t39 * t18;
        let t49 = t22 * t10 * t17;
        let t51 = t42 * t17;
        let t52 = t8 * t51;
        let t55 = t35 * t22 * t24;
        let t58 = 1.0 / t21 / t6;
        let t59 = t58 * t24;
        let t61 = t36 * t29;
        let t63 = t43 * t29;
        let t65 = t39 * t32;
        let t67 = t22 * t32;
        let t69 = 0.17722691847124394 * t37 + 0.3544538369424879 * t44 + 0.19997916265148655 * t46 - 0.7853981633974483 * t49 + 0.3999583253029731 * t52 + 0.08861345923562197 * t55 - 0.6960409996039635 * t59 + 0.3544538369424879 * t61 + 0.7089076738849758 * t63 + 0.19997916265148655 * t65 - 0.7853981633974483 * t67;
        let tvrho0 = t1 * t69 + t16 + t20 + t26 + t31 + t34;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t83 = 1.0 / t2 / t1;
        let t84 = t83 * t10;
        let t85 = t84 * t14;
        let t87 = t35 * t42;
        let t88 = t87 * t14;
        let t90 = t83 * t7;
        let t92 = 1.0 / t1;
        let t93 = t92 * t22;
        let t97 = -0.98185 * t90 - 3.85611769 * t93 + 30.289033231412 * t58 * t35;
        let t98 = t2 * t97;
        let t99 = t98 * t14;
        let t101 = t90 * t18;
        let t103 = t93 * t18;
        let t105 = t39 * t51;
        let t108 = t17 * t35;
        let t109 = t58 * t10 * t108;
        let t112 = t22 * t42 * t17;
        let t114 = t97 * t17;
        let t115 = t8 * t114;
        let t118 = t83 * t22 * t24;
        let t121 = t92 * t58 * t24;
        let t123 = t21 * t21;
        let t124 = 1.0 / t123;
        let t126 = t124 * t24 * t35;
        let t128 = t84 * t29;
        let t130 = t87 * t29;
        let t132 = t98 * t29;
        let t134 = t90 * t32;
        let t136 = t93 * t32;
        let t139 = t58 * t32 * t35;
        let t141 = -0.08861345923562197 * t85 + 0.3544538369424879 * t88 + 0.3544538369424879 * t99 - 0.09998958132574327 * t101 - 0.39269908169872414 * t103 + 0.3999583253029731 * t105 + 3.0845727469271385 * t109 - 1.5707963267948966 * t112 + 0.3999583253029731 * t115 - 0.044306729617810986 * t118 - 0.34802049980198174 * t121 + 4.100447132766909 * t126 - 0.17722691847124394 * t128 + 0.7089076738849758 * t130 + 0.7089076738849758 * t132 - 0.09998958132574327 * t134 - 0.39269908169872414 * t136 + 3.0845727469271385 * t139;
        let tv2rho20 = 0.3544538369424879 * t37 + 0.7089076738849758 * t44 + 0.3999583253029731 * t46 - 1.5707963267948966 * t49 + 0.7999166506059462 * t52 + 0.17722691847124394 * t55 - 1.392081999207927 * t59 + 0.7089076738849758 * t61 + 1.4178153477699516 * t63 + 0.3999583253029731 * t65 - 1.5707963267948966 * t67 + t1 * t141;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t161 = t1 * t1;
        let t163 = 1.0 / t2 / t161;
        let t164 = t163 * t7;
        let t165 = t164 * t18;
        let t167 = t90 * t51;
        let t169 = 1.0 / t161;
        let t170 = t169 * t22;
        let t171 = t170 * t18;
        let t173 = t93 * t51;
        let t175 = t83 * t58;
        let t176 = t175 * t18;
        let t178 = t39 * t114;
        let t181 = t58 * t42 * t108;
        let t184 = t17 * t92;
        let t185 = t124 * t10 * t184;
        let t191 = 1.472775 * t164 + 5.784176535 * t170 - 178.43572366957125 * t124 * t92;
        let t192 = t191 * t17;
        let t193 = t8 * t192;
        let t195 = t163 * t10;
        let t196 = t195 * t14;
        let t199 = t163 * t22 * t24;
        let t201 = t195 * t29;
        let t203 = 0.14998437198861492 * t165 - 0.29996874397722983 * t167 + 0.5890486225480862 * t171 - 1.1780972450961724 * t173 + 1e-19 * t176 + 0.5999374879544597 * t178 + 9.253718240781415 * t181 - 18.171526509422467 * t185 + 0.3999583253029731 * t193 + 0.13292018885343296 * t196 + 0.06646009442671648 * t199 + 0.2658403777068659 * t201;
        let t204 = t164 * t32;
        let t206 = t83 * t42;
        let t207 = t206 * t14;
        let t209 = t35 * t97;
        let t210 = t209 * t14;
        let t212 = t2 * t191;
        let t213 = t212 * t14;
        let t216 = t22 * t97 * t17;
        let t219 = t169 * t58 * t24;
        let t222 = 1.0 / t123 / t6;
        let t223 = t222 * t24;
        let t224 = t223 * t92;
        let t226 = t206 * t29;
        let t228 = t209 * t29;
        let t230 = t212 * t29;
        let t232 = t170 * t32;
        let t234 = t175 * t32;
        let t237 = t124 * t32 * t92;
        let t239 = 0.14998437198861492 * t204 - 0.2658403777068659 * t207 + 0.5316807554137318 * t210 + 0.3544538369424879 * t213 - 2.356194490192345 * t216 + 0.5220307497029726 * t219 - 32.20819213845752 * t224 - 0.5316807554137318 * t226 + 1.0633615108274637 * t228 + 0.7089076738849758 * t230 + 0.5890486225480862 * t232 + 1e-19 * t234 - 18.171526509422467 * t237;
        let tv3rho30 = -0.29996874397722983 * t101 - 0.2658403777068659 * t85 + 1.1998749759089193 * t105 - 1.1780972450961724 * t103 + 9.253718240781415 * t109 + 1.1998749759089193 * t115 - 0.13292018885343296 * t118 - 0.5316807554137318 * t128 - 0.29996874397722983 * t134 + 1.0633615108274637 * t88 + 1.0633615108274637 * t99 - 4.71238898038469 * t112 - 1.0440614994059452 * t121 + 12.301341398300728 * t126 + 2.1267230216549273 * t130 + 2.1267230216549273 * t132 - 1.1780972450961724 * t136 + 9.253718240781415 * t139 + t1 * (t203 + t239);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
