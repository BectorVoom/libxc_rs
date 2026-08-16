//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1051/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1051(t11238: f64, t11435: f64, t345: f64, t242: f64, t947: f64, t8431: f64, t8435: f64, t8439: f64, t8453: f64, t8456: f64, t8462: f64, t8472: f64, t8481: f64, t8484: f64, t8500: f64, t946: f64) -> (f64, f64) {
    let t11436 = t11238 + t11435;
    let t11437 = t11436 * t345;
    let t11439 = t242 * t947 * t11437;
    let t11452 = t946 * t11439 / 3072.0_f64 + t8431 / 4608.0_f64 + t8435 / 2304.0_f64 - t8439 / 4608.0_f64 - t8453 / 162.0_f64 - t8456 / 648.0_f64 - t8462 / 648.0_f64 - t8472 / 6912.0_f64 + t8481 / 6912.0_f64 + t8484 / 648.0_f64 - t8500 / 432.0_f64;
    (t11436, t11452)
}
