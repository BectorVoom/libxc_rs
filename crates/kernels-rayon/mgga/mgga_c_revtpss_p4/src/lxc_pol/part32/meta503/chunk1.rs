//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1791/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1791(t25237: f64, t5989: f64, t5993: f64, t7045: f64, t5985: f64, t7025: f64, t6019: f64, t7038: f64, t6030: f64, t1558: f64, t1579: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29623 = t25237 * t5989;
    let t29627 = t7045 * t5993;
    let t29629 = t7025 * t5985;
    let t29631 = t7038 * t6019;
    let t29633 = t7045 * t6030;
    let t29682 = t1579 * t1558 * t231;
    (t29623, t29627, t29629, t29631, t29633, t29682)
}
