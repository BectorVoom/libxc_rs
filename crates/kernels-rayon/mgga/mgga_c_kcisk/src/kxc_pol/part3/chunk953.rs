//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 953/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk953(t14013: f64, t14045: f64, t14081: f64, t14109: f64, t1216: f64, t1349: f64, t13966: f64, t13967: f64, t13969: f64, t13971: f64, t13973: f64, t13975: f64, t13978: f64, t13982: f64, t1402: f64, t338: f64, t3729: f64, t3904: f64, t417: f64) -> (f64, f64) {
    let t14111 = t14013 + t14045 + t14081 + t14109;
    let t14115 = -t13966 - 0.14055920378328537299e-1_f64 * t13967 - 0.28111840756657074597e-1_f64 * t13969 + 0.70279601891642686494e-2_f64 * t13971 + 0.14055920378328537299e-1_f64 * t13973 - 0.23426533963880895498e-2_f64 * t1349 * t13975 - 0.46853067927761790996e-2_f64 * t417 * t13978 - 0.42167761134985611897e-1_f64 * t1349 * t13982 - 3.0_f64 * t3729 * t1402 - t338 * t14111 - 3.0_f64 * t1216 * t3904;
    (t14111, t14115)
}
