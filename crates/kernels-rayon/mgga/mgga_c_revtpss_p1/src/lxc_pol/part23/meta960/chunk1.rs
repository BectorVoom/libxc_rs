//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3234/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3234(t1471: f64, t1487: f64, t1494: f64, t21769: f64, t21805: f64, t22718: f64, t22739: f64, t4188: f64, t4191: f64, t4217: f64, t4218: f64, t4238: f64, t5819: f64, t5855: f64, t5869: f64, t607: f64, t628: f64, t71: f64, t77: f64, t85: f64, t85125: f64) -> f64 {
    let t85141 = t21769 * t1494 / 8.0_f64 + t5855 * t4238 / 8.0_f64 + t4218 * t5869 / 8.0_f64 + t1487 * t21805 / 8.0_f64 + t628 * t22739 / 24.0_f64 + t71 * t77 * t85125 / 24.0_f64 - t5819 * t4217 * t85 / 4.0_f64 - t4188 * t5869 / 4.0_f64 - t4191 * t5869 / 4.0_f64 - t1471 * t21805 / 4.0_f64 - t607 * t22718 * t85 / 12.0_f64;
    t85141
}
