//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3164/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3164(t12916: f64, t17743: f64, t3718: f64, t12881: f64, t5391: f64, t1222: f64, t16720: f64, t17471: f64, t17753: f64, t17755: f64, t12800: f64, t5378: f64) -> (f64, f64, f64, f64, f64) {
    let t57386 = t3718 * t12916 * t17743;
    let t57421 = t5391 * t12881;
    let t57428 = t1222 * t17471 * t16720;
    let t57435 = t17753 * t12916 * t17755;
    let t57449 = t12800 * t5378;
    (t57386, t57421, t57428, t57435, t57449)
}
