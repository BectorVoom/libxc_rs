//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1036/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1036<F: Float>(t1056: F, t2191: F, t1364: F, t1354: F, t3819: F, t2059: F, t3831: F, t3593: F, t1391: F, t5643: F, t443: F, t5647: F, t2181: F, t3283: F, t1349: F, t13967: F, t13969: F, t13971: F, t13973: F, t19424: F, t19710: F, t20792: F, t20796: F, t20798: F, t20800: F, t20803: F, t20806: F, t20812: F, t2110: F, t2209: F, t338: F, t3729: F, t3904: F, t417: F, t451: F) -> (F,) {
    let t20813 = t2191 * t1056;
    let t20814 = t20813 * t1364;
    let t20817 = t3819 * t1354;
    let t20820 = t3831 * t2059;
    let t20821 = t20820 * t3593;
    let t20825 = 0.93706135855523581992e-2 * t1391 * t5643;
    let t20827 = 0.93706135855523581992e-2 * t443 * t5647;
    let t20828 = t2181 * t3283;
    let t20831 = -t338 * t20792 + 0.46853067927761790996e-2 * t13973 - t19710 * t451 - 0.46853067927761790996e-2 * t20796 - 0.93706135855523581992e-2 * t20798 - 0.46853067927761790996e-2 * t417 * t20800 + 0.23426533963880895498e-1 * t20803 + t20806 - t3729 * t2209 - t2110 * t3904 - 0.93706135855523581992e-2 * t13967 - 0.18741227171104716398e-1 * t13969 + 0.23426533963880895498e-2 * t13971 - 0.28111840756657074598e-1 * t20812 * t20814 - 0.18741227171104716398e-1 * t20817 * t19424 - 0.14055920378328537299e-1 * t1349 * t20821 - t20825 - t20827 + 0.46853067927761790996e-2 * t3819 * t20828;
    (t20831,)
}
