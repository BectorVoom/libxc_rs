//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 669/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk669(t1416: f64, t3989: f64, t1386: f64, t240: f64, t1398: f64, t543: f64, t550: f64) -> (f64, f64, f64) {
    let t3990 = t3989 * t1416;
    let t3992 = t1386 * t240;
    let t3994 = t550 * t1398 * t543;
    (t3990, t3992, t3994)
}
