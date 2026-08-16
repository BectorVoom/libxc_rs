//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2794/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2794(t2737: f64, t40609: f64, t2694: f64, t9789: f64, t853: f64, t9794: f64, t775: f64, t837: f64, t10760: f64, t10292: f64, t66: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40611 = 0.63807336860547134325e-3_f64 * t40609 * t2737;
    let t40625 = t9789 * t2694;
    let t40627 = t9794 * t853;
    let t40628 = t837 * t775;
    let t40630 = t10760 * t40627 * t40628;
    let t40633 = 1.0_f64 / t66 / t10292;
    let t40634 = t40633 * t240;
    (t40611, t40625, t40627, t40628, t40630, t40633, t40634)
}
