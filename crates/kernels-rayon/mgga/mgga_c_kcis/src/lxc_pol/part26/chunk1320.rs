//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1320/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1320(t28524: f64, t303: f64, t5628: f64, t1014: f64, t29304: f64, t102280: f64, t102626: f64, t102629: f64, t102632: f64, t102636: f64, t27583: f64, t95137: f64, t99591: f64, t99593: f64, t99600: f64, t99610: f64) -> (f64, f64, f64) {
    let t102640 = t303 * t28524 * t5628;
    let t102642 = t1014 * t29304;
    let t102646 = -0.15476481481481481481e-2_f64 * t102626 + 0.46429444444444444444e-2_f64 * t102629 - t99591 - t99593 + t99600 + 0.17411041666666666666e-2_f64 * t102632 - 0.61905925925925925924e-2_f64 * t102636 + 0.7722800925925925926e-4_f64 * t95137 - 0.34822083333333333332e-2_f64 * t102640 - 0.41270617283950617283e-2_f64 * t102642 - 0.46336805555555555556e-3_f64 * t27583 * t102280 - t99610;
    (t102640, t102642, t102646)
}
