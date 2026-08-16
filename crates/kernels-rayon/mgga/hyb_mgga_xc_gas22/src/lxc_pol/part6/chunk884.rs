//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 884/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk884(t2631: f64, t7497: f64, t1057: f64, t2814: f64, t1068: f64, t2751: f64, t1100: f64, t2696: f64, t462: f64, t10: f64, t1107: f64, t1095: f64, t2639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7498 = t7497 * t2631;
    let t7503 = 12.0_f64 * t1057 * t2814;
    let t7506 = t2751 * t1068;
    let t7508 = t2696 * t1100;
    let t7509 = t462 * t7508;
    let t7511 = t2696 * t10;
    let t7512 = t7511 * t1107;
    let t7515 = t2639 * t1095;
    (t7498, t7503, t7506, t7508, t7509, t7511, t7512, t7515)
}
