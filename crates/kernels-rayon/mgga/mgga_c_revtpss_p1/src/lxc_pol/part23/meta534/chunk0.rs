//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2064/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2064(t21804: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t21727: f64, t21769: f64, t4188: f64, t4191: f64, t4196: f64, t4218: f64, t4238: f64, t5855: f64, t5869: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64) -> (f64, f64) {
    let t21805 = t77 * t21804;
    let t21808 = -t4188 * t1494 / 6.0_f64 - t4191 * t1494 / 6.0_f64 - t1471 * t4238 / 6.0_f64 - t21727 * t85 / 12.0_f64 + t21769 * t85 / 24.0_f64 + t5855 * t641 / 24.0_f64 - t4196 * t1494 / 6.0_f64 + t4218 * t1494 / 12.0_f64 + t1487 * t4238 / 12.0_f64 - t608 * t5869 / 12.0_f64 + t628 * t5869 / 24.0_f64 + t71 * t21805 / 24.0_f64;
    (t21805, t21808)
}
