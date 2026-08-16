//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2198/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2198(t651: f64, t6765: f64, t7002: f64, t28167: f64, t86753: f64, t8717: f64, t13648: f64, t2014: f64, t7934: f64, t29589: f64, t7235: f64, t13426: f64, t7742: f64) -> (f64, f64, f64, f64, f64) {
    let t109029 = 2.0_f64 * t651 * t6765 * t7002;
    let t109035 = 6.0_f64 * t28167 * t8717 * t86753;
    let t109038 = 2.0_f64 * t2014 * t7934 * t13648;
    let t109039 = t7235 * t29589;
    let t109041 = 4.0_f64 * t13426 * t7742;
    (t109029, t109035, t109038, t109039, t109041)
}
