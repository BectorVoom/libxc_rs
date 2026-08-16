//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1053/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1053(t3827: f64, t588: f64, t3029: f64, t3042: f64, t3047: f64, t3052: f64, t3057: f64, t3062: f64, t584: f64, t9954: f64, t9959: f64, t9962: f64, t9967: f64, t9970: f64, t9975: f64, t9978: f64, t9983: f64, t9986: f64, t9991: f64) -> (f64, f64) {
    let t9994 = t588 * t3827;
    let t9997 = t9954 * t584 / 1152.0_f64 - t3042 * t3029 / 5760.0_f64 - t9959 * t584 / 11520.0_f64 - t9962 * t584 / 21504.0_f64 + t3047 * t3029 / 129024.0_f64 + t9967 * t584 / 258048.0_f64 + t9970 * t584 / 491520.0_f64 - t3052 * t3029 / 3440640.0_f64 - t9975 * t584 / 6881280.0_f64 - t9978 * t584 / 13271040.0_f64 + t3057 * t3029 / 0.10616832e9_f64 + t9983 * t584 / 0.21233664e9_f64 + t9986 * t584 / 412876800.0_f64 - t3062 * t3029 / 0.37158912e10_f64 - t9991 * t584 / 0.74317824e10_f64 - 2.0_f64 / 3.0_f64 * t9994 * t584;
    (t9994, t9997)
}
