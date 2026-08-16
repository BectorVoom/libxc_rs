//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1252/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1252(t3446: f64, t3453: f64, t9066: f64, t9069: f64, t9072: f64, t10648: f64, t10649: f64, t11582: f64, t2768: f64, t3033: f64, t58: f64, t597: f64) -> (f64, f64, f64, f64, f64) {
    let t43892 = t3446 * t3453 * t9066;
    let t43895 = t3446 * t3453 * t9069;
    let t43898 = t3446 * t3453 * t9072;
    let t43902 = t10648 * t10649 * t11582 * t2768;
    let t43907 = t10648 * t10649 * t58 * t3033 * t597;
    (t43892, t43895, t43898, t43902, t43907)
}
