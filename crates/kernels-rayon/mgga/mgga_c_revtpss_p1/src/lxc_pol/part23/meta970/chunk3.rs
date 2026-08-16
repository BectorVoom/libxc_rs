//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3273/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3273(t221: f64, t22954: f64, t4018: f64, t4019: f64, t22893: f64, t2661: f64, t3992: f64, t48455: f64, t22858: f64, t47293: f64, t10001: f64, t22863: f64) -> (f64, f64, f64, f64) {
    let t86061 = t4018 * t4019 * t221 * t22954;
    let t86070 = t2661 * t3992 * t48455 * t22893;
    let t86074 = t47293 * t4019 * t221 * t22858;
    let t86078 = t10001 * t4019 * t221 * t22863;
    (t86061, t86070, t86074, t86078)
}
