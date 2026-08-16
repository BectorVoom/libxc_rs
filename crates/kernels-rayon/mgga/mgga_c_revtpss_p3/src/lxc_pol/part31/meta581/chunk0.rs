//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2001/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2001(t3114: f64, t93596: f64, t11880: f64, t7111: f64, t11817: f64, t7117: f64, t3204: f64, t7125: f64, t11788: f64, t1972: f64, t3080: f64, t7106: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93670 = t3114 * t93596;
    let t93696 = t7111 * t11880;
    let t93720 = t7117 * t11817;
    let t93728 = t3204 * t7125;
    let t93731 = t11788 * t1972;
    let t93745 = t7106 * t3080;
    (t93670, t93696, t93720, t93728, t93731, t93745)
}
