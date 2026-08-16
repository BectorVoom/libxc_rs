//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1755/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1755(t4003: f64, t5658: f64, t1448: f64, t1868: f64, t2007: f64, t2371: f64, t197: f64, t531: f64, t2013: f64) -> (f64, f64, f64, f64, f64) {
    let t21990 = t4003 * t5658;
    let t22496 = t1868 * t1448;
    let t25078 = t2007 * t2371;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t21990, t22496, t25078, t25081, t25082)
}
