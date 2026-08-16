//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1191/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1191(t1068: f64, t5891: f64, t1067: f64, t462: f64, t7482: f64, t21837: f64, t2731: f64, t7441: f64, t1037: f64, t1046: f64, t2728: f64, t7453: f64) -> (f64, f64, f64, f64) {
    let t22061 = t5891 * t1068;
    let t22064 = t462 * t1067 * t7482;
    let t22068 = 0.57895126195293126241e3_f64 * t7441 * t21837 * t2731;
    let t22072 = 0.34367190188705947437e1_f64 * t1037 * t2728 * t1046 * t7453;
    (t22061, t22064, t22068, t22072)
}
