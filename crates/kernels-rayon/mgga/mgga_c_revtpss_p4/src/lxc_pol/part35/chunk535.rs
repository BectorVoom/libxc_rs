//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 535/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk535(t2299: f64, t5819: f64, t5825: f64, t633: f64, t2306: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t5820: f64, t5827: f64, t5830: f64, t5855: f64, t71: f64, t85: f64) -> (f64, f64, f64) {
    let t5860 = t2299 * t5819;
    let t5862 = t633 * t5825;
    let t5864 = t2306 * t5819;
    let t5866 = t637 * t5825;
    let t5868 = 28.0_f64 / 9.0_f64 * t5860 - 4.0_f64 / 3.0_f64 * t5862 + 28.0_f64 / 9.0_f64 * t5864 + 4.0_f64 / 3.0_f64 * t5866;
    let t5869 = t77 * t5868;
    let t5872 = -t5820 * t85 / 12.0_f64 - t5827 * t85 / 12.0_f64 - t5830 * t85 / 6.0_f64 - t1471 * t1494 / 6.0_f64 + t5855 * t85 / 24.0_f64 + t1487 * t1494 / 12.0_f64 + t71 * t5869 / 24.0_f64;
    (t5868, t5869, t5872)
}
