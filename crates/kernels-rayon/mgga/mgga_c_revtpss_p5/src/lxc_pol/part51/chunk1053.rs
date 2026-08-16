//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1053/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1053(t1032: f64, t3057: f64, t32013: f64, t42058: f64, t8512: f64, t31991: f64, t378: f64, t120190: f64, t32009: f64, t31883: f64, t31909: f64, t7165: f64) -> (f64, f64, f64, f64, f64) {
    let t120466 = t3057 * t1032 * t32013;
    let t120471 = t8512 * t42058;
    let t120473 = t120471 * t378 * t31991;
    let t120476 = t32009 * t120190;
    let t120479 = t31909 * t31883;
    let t120481 = t120471 * t7165;
    (t120466, t120473, t120476, t120479, t120481)
}
