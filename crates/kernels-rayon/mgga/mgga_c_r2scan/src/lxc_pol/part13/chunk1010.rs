//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1010/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1010(t11665: f64, t11689: f64, t11723: f64, t11755: f64, t11779: f64, t11814: f64, t11833: f64, t11852: f64, t797: f64, t1048: f64, t499: f64, t10991: f64, t10996: f64, t11008: f64, t11018: f64, t11549: f64, t11553: f64, t11558: f64, t11562: f64, t11616: f64, t11619: f64, t11624: f64, t11628: f64, t11632: f64, t11635: f64, t11638: f64) -> (f64, f64, f64, f64) {
    let t11855 = t11665 + t11689 + t11723 + t11755 + t11779 + t11814 + t11833 + t11852;
    let t11856 = t11855 * t797;
    let t11858 = t1048 * t499 * t11856;
    let t11859 = t11858 / 4.0_f64;
    let t11860 = t11549 - t11553 - t11558 - 0.40650199722100037752e-3_f64 * t11616 + t10991 + t10996 - t11562 + t11619 - 0.40650199722100037752e-3_f64 * t11008 - t11624 - t11018 - t11628 + t11632 + t11635 - t11638 - t11859;
    (t11855, t11856, t11858, t11860)
}
