//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 955/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk955(t1354: f64, t3283: f64, t1364: f64, t3619: f64, t3823: f64, t3830: f64, t423: f64, t12873: f64, t3831: f64, t13125: f64, t1349: f64, t14093: f64, t14116: f64, t14118: f64, t14120: f64, t14122: f64, t14126: f64, t14129: f64, t3819: f64, t417: f64, t451: f64) -> f64 {
    let t14132 = t1354 * t3283;
    let t14133 = t14132 * t1364;
    let t14136 = t3823 * t3619;
    let t14140 = 1.0_f64 / t3830 / t423;
    let t14141 = t14140 * t12873;
    let t14145 = t3831 * t1364 * t3619;
    let t14149 = -0.14055920378328537299e-1_f64 * t14116 - 0.28111840756657074597e-1_f64 * t14118 - 0.42167761134985611897e-1_f64 * t14120 - 0.14055920378328537299e-1_f64 * t14093 * t14122 - 0.28111840756657074597e-1_f64 * t3819 * t14126 + 0.14055920378328537299e-1_f64 * t3819 * t14129 + 0.14055920378328537299e-1_f64 * t1349 * t14133 + 0.14055920378328537299e-1_f64 * t1349 * t14136 - 0.56223681513314149196e-1_f64 * t417 * t14141 + 0.42167761134985611897e-1_f64 * t417 * t14145 - t13125 * t451;
    t14149
}
