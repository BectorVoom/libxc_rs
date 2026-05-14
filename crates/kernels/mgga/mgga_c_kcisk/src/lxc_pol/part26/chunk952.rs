//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 952/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk952<F: Float>(t2059: F, t2191: F, t1056: F, t1364: F, t1216: F, t1349: F, t13967: F, t13969: F, t1402: F, t20796: F, t20798: F, t20803: F, t20806: F, t20812: F, t20817: F, t2110: F, t25554: F, t25557: F, t25559: F, t25561: F, t25563: F, t25565: F, t25657: F, t25906: F, t338: F, t417: F, t451: F, t5864: F, t7828: F, t8159: F) -> (F, F, F, F) {
    let t25911 = t2059 * t2191;
    let t25912 = t25911 * t1056;
    let t25915 = t25911 * t1364;
    let t25920 = -t338 * t25554 - t1216 * t8159 + 0.46853067927761790996e-2 * t25557 - 0.46853067927761790996e-2 * t25559 + 0.23426533963880895498e-2 * t25561 - 0.14055920378328537299e-1 * t25563 - 0.23426533963880895498e-2 * t1349 * t25565 - 0.46853067927761790996e-2 * t417 * t25657 - 2.0 * t2110 * t5864 - t7828 * t1402 - t25906 * t451 - 0.93706135855523581992e-2 * t20796 - 0.18741227171104716398e-1 * t20798 + 0.93706135855523581992e-2 * t20803 + t20806 - 0.18741227171104716398e-1 * t20817 * t25912 - 0.28111840756657074598e-1 * t20812 * t25915 - 0.46853067927761790996e-2 * t13967 - 0.93706135855523581992e-2 * t13969;
    (t25911, t25912, t25915, t25920)
}
