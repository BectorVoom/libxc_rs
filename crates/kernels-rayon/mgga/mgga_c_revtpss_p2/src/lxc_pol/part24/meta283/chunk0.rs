//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1059/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1059(t11465: f64, t6189: f64, t3336: f64, t6396: f64, t6184: f64, t964: f64, t6152: f64, t945: f64, t11387: f64, t6109: f64, t2970: f64, t6173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19133 = t11465 * t6189;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    let t19173 = t6152 * t945;
    let t19255 = t6109 * t11387;
    let t19275 = t6173 * t2970;
    (t19133, t19153, t19156, t19173, t19255, t19275)
}
