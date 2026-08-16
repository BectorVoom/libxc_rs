//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1346/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1346(t114800: f64, t25082: f64, t8717: f64, t1450: f64, t2014: f64, t2033: f64, t22813: f64, t22633: f64, t94: f64, t1937: f64, t29508: f64, t7735: f64) -> (f64, f64, f64, f64) {
    let t114803 = 9.0_f64 * t25082 * t8717 * t114800;
    let t114807 = 6.0_f64 * t2014 * t22813 * t2033 * t1450;
    let t114812 = t94 * t22633;
    let t114814 = 2.0_f64 * t114812 * t1937;
    let t114816 = 6.0_f64 * t29508 * t7735;
    (t114803, t114807, t114814, t114816)
}
