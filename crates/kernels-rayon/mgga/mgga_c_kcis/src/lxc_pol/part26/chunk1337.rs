//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1337/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1337(t22354: f64, t27544: f64, t8191: f64, t97701: f64, t28589: f64, t5932: f64, t22403: f64, t7948: f64, t28610: f64, t5910: f64, t102889: f64, t102892: f64, t102894: f64, t102896: f64, t102898: f64, t102900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102902 = t27544 * t22354;
    let t102904 = t97701 * t8191;
    let t102906 = t28589 * t5932;
    let t102908 = t7948 * t22403;
    let t102910 = t28610 * t5910;
    let t102912 = 0.25e0_f64 * t102889 + 0.43166666666666666668e0_f64 * t102892 + 0.53958333333333333333e-1_f64 * t102894 + 0.53958333333333333334e-1_f64 * t102896 + 0.20234375e-1_f64 * t102898 - 1.0_f64 * t102900 + 0.17986111111111111111e-1_f64 * t102902 - 0.1875e0_f64 * t102904 - 0.125e0_f64 * t102906 - 0.625e-1_f64 * t102908 + 0.11111111111111111111e0_f64 * t102910;
    (t102902, t102904, t102906, t102908, t102910, t102912)
}
