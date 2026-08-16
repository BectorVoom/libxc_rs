//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1699/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1699(t4131: f64, t676: f64, t123: f64, t3915: f64, t2453: f64, t3914: f64) -> (f64, f64, f64) {
    let t9670 = t676 * t4131;
    let t9671 = t123 * t9670;
    let t9672 = t3915 * t9671;
    let t9674 = t2453 * t3914;
    (t9671, t9672, t9674)
}
