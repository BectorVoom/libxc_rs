//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2211/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2211(t30004: f64, t5523: f64, t27833: f64, t7935: f64, t1448: f64, t6922: f64, t28196: f64, t28197: f64, t28067: f64, t98450: f64, t7897: f64, t8995: f64) -> (f64, f64, f64, f64, f64) {
    let t109256 = 2.0_f64 * t5523 * t30004;
    let t109262 = 2.0_f64 * t27833 * t7935;
    let t109263 = t6922 * t1448;
    let t109266 = 2.0_f64 * t28196 * t28197 * t109263;
    let t109268 = 6.0_f64 * t98450 * t28067;
    let t109269 = t7897 * t8995;
    (t109256, t109262, t109266, t109268, t109269)
}
