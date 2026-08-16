//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1828/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1828(t3151: f64, t3153: f64, t3154: f64, t905: f64, t606: f64, t1052: f64, t360: f64) -> (f64, f64, f64, f64) {
    let t11659 = t3151 * t3153;
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    let t11670 = t360 * t1052;
    (t11659, t11660, t11661, t11670)
}
