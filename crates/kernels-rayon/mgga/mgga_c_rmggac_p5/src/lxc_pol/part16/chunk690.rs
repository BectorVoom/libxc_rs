//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 690/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk690(t1550: f64, t9951: f64, t1756: f64, t2060: f64, t739: f64, t515: f64, t6522: f64, t3352: f64, t3351: f64, t2286: f64, t8571: f64, t558: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9952 = t1550 * t9951;
    let t9957 = t2060 * t1756;
    let t9958 = t739 * t9957;
    let t9963 = t515 * t6522;
    let t9964 = t3352 * t9963;
    let t9965 = t3351 * t9964;
    let t9967 = t8571 * t2286;
    let t9969 = t558 * t615;
    (t9952, t9957, t9958, t9964, t9965, t9967, t9969)
}
