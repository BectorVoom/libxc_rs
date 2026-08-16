//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 837/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk837(t762: f64, t771: f64, t777: f64, t2838: f64, t883: f64, t2958: f64, t682: f64, t691: f64, t680: f64, t272: f64, t286: f64, t791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11708 = 36.0_f64 * t777 * t762 * t771;
    let t11721 = t883 * t2838;
    let t11731 = t2958 * t682;
    let t11733 = t2958 * t691;
    let t11735 = t680 * t680;
    let t11739 = 0.35089341735807877242e1_f64 * t286 * t791 * t11735 * t272;
    (t11708, t11721, t11731, t11733, t11735, t11739)
}
