//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1834/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1834(t1071: f64, t3140: f64, t1078: f64, t1982: f64, t7135: f64, t988: f64, t7145: f64, t1976: f64, t3057: f64) -> (f64, f64, f64) {
    let t25638 = t1071 * t3140;
    let t25640 = t1982 * t25638 * t1078;
    let t25647 = t7135 * t988;
    let t25648 = t7145 * t25647;
    let t25651 = t3057 * t1976;
    (t25640, t25648, t25651)
}
