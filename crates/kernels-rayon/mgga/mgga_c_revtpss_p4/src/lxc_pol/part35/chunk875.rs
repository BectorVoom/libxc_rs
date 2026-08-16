//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 875/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk875(t1471: f64, t1487: f64, t1494: f64, t21686: f64, t22662: f64, t22665: f64, t22673: f64, t22676: f64, t22681: f64, t22719: f64, t22739: f64, t5820: f64, t5827: f64, t5830: f64, t5855: f64, t5869: f64, t71: f64, t85: f64) -> f64 {
    let t22742 = -t21686 * t22662 / 4.0_f64 - t22665 * t85 / 4.0_f64 - t5820 * t1494 / 4.0_f64 - t22673 * t85 / 12.0_f64 - t22676 * t85 / 4.0_f64 - t5827 * t1494 / 4.0_f64 - t22681 * t85 / 4.0_f64 - t5830 * t1494 / 2.0_f64 - t1471 * t5869 / 4.0_f64 + t22719 * t85 / 24.0_f64 + t5855 * t1494 / 8.0_f64 + t1487 * t5869 / 8.0_f64 + t71 * t22739 / 24.0_f64;
    t22742
}
