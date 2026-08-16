//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1462/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1462(t104990: f64, t106921: f64, t106923: f64, t106932: f64, t106934: f64, t106937: f64, t106939: f64, t106941: f64, t106953: f64, t106958: f64, t106960: f64, t108902: f64, t109029: f64, t1458: f64, t20347: f64, t27863: f64, t33690: f64, t5493: f64, t7266: f64) -> f64 {
    let t109976 = 6.0_f64 * t104990 * t1458 + 2.0_f64 * t20347 * t7266 + 6.0_f64 * t27863 * t5493 + 6.0_f64 * t33690 * t5493 + t106921 + t106923 + t106932 + t106934 + t106937 + t106939 + t106941 + t106953 + t106958 + t106960 + 6.0_f64 * t108902 + t109029;
    t109976
}
