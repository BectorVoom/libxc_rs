//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2455/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2455(t3115: f64, t3119: f64, t42793: f64, t11688: f64, t11922: f64, t4892: f64, t11249: f64, t3151: f64, t11722: f64, t3188: f64, t3046: f64, t3316: f64, t4891: f64) -> (f64, f64, f64, f64, f64) {
    let t42795 = t3115 * t42793 * t3119;
    let t42798 = t4892 * t11922 * t11688;
    let t42804 = t3151 * t11249;
    let t42816 = t3188 * t11722;
    let t42830 = t3046 * t3316 * t4891;
    (t42795, t42798, t42804, t42816, t42830)
}
