//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1036/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1036(t137: f64, t14423: f64, t1165: f64, t5012: f64, t7564: f64, t1181: f64, t30209: f64, t5099: f64, t604: f64, t4342: f64, t7575: f64, t8600: f64) -> (f64, f64, f64) {
    let t34248 = t14423 * t137;
    let t34251 = t7564 * t1165 * t34248 * t5012;
    let t34255 = t30209 * t1181 * t604 * t5099;
    let t34259 = t7575 * t1165 * t8600 * t4342;
    (t34251, t34255, t34259)
}
