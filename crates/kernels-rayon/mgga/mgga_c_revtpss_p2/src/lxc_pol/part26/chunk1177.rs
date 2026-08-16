//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1177/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1177(t95667: f64, t95682: f64, t95698: f64, t95713: f64, t26482: f64, t93321: f64, t25375: f64, t95628: f64, t136: f64, t137: f64, t2061: f64, t10505: f64) -> (f64, f64, f64, f64, f64) {
    let t95715 = t95667 + t95682 + t95698 + t95713;
    let t95720 = t93321 * t26482;
    let t95722 = t25375 * t95628;
    let t95725 = t2061 * t136 * t137;
    let t95726 = t95725 * t10505;
    (t95715, t95720, t95722, t95725, t95726)
}
