//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2374/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2374(t3601: f64, t3603: f64, t17710: f64, t3720: f64, t13127: f64, t17708: f64) -> (f64, f64, f64, f64) {
    let t17748 = t3601 * t3603;
    let t17749 = t17710 * t17748;
    let t17750 = t3720 * t17749;
    let t17753 = t13127 * t17708;
    (t17748, t17749, t17750, t17753)
}
