//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1312/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1312(t1459: f64, t303: f64, t7203: f64, t29600: f64, t7974: f64, t102464: f64, t102467: f64, t102475: f64, t102478: f64, t102481: f64, t28749: f64, t28755: f64, t95088: f64, t98573: f64, t99301: f64) -> (f64, f64) {
    let t102484 = t303 * t1459 * t7203;
    let t102486 = t29600 * t7974;
    let t102488 = t95088 - 0.17411041666666666666e-2_f64 * t102464 + 0.34822083333333333332e-2_f64 * t102467 + 0.61905925925925925925e-2_f64 * t98573 + 0.23168402777777777778e-3_f64 * t99301 * t28749 + 0.23168402777777777778e-3_f64 * t99301 * t28755 - 0.17024129629629629629e-1_f64 * t102475 + 0.11349419753086419753e-1_f64 * t102478 - 0.61905925925925925925e-2_f64 * t102481 + 0.11607361111111111111e-2_f64 * t102484 - 0.11326774691358024691e-2_f64 * t102486;
    (t102484, t102488)
}
