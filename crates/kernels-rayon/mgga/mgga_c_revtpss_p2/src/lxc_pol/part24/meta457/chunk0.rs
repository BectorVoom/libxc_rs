//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1426/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1426(t378: f64, t53014: f64, t11200: f64, t1678: f64, t11970: f64, t1660: f64, t127: f64, t4823: f64, t11239: f64, t1647: f64, t11245: f64, t11255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53015 = t53014 * t378;
    let t53160 = t11200 * t1678;
    let t53326 = t1660 * t11970;
    let t53391 = t127 * t4823;
    let t53703 = t1647 * t11239;
    let t53704 = t53703 * t11245;
    let t53707 = t53703 * t11255;
    (t53015, t53160, t53326, t53391, t53703, t53704, t53707)
}
