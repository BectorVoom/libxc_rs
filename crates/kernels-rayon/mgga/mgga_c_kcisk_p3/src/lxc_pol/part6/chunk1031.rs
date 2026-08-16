//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1031/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1031(t1354: f64, t30941: f64, t1349: f64, t13966: f64, t20796: f64, t20798: f64, t25557: f64, t25559: f64, t25561: f64, t25563: f64, t25925: f64, t30900: f64, t30909: f64, t417: f64) -> f64 {
    let t30942 = t1354 * t30941;
    let t30946 = -0.42167761134985611897e-1_f64 * t1349 * t30900 - t13966 - 0.14055920378328537299e-1_f64 * t20796 - 0.28111840756657074597e-1_f64 * t20798 + 0.14055920378328537299e-1_f64 * t25557 - 0.14055920378328537299e-1_f64 * t25559 + 0.70279601891642686494e-2_f64 * t25561 - 0.42167761134985611897e-1_f64 * t25563 - 0.23426533963880895498e-2_f64 * t1349 * t30909 - 0.46853067927761790996e-2_f64 * t417 * t30942 - 0.28111840756657074597e-1_f64 * t25925;
    t30946
}
