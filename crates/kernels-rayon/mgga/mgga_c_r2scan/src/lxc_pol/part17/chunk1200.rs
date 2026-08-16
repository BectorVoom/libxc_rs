//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1200/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1200(t1053: f64, t1102: f64, t1103: f64, t9005: f64, t10680: f64, t11587: f64, t40310: f64, t10673: f64, t11591: f64, t40317: f64, t3446: f64, t3453: f64, t9063: f64) -> (f64, f64, f64, f64) {
    let t43854 = t1102 * t1053 * t1103 * t9005;
    let t43875 = t10680 * t11587 * t40310;
    let t43878 = t10673 * t11591 * t40317;
    let t43887 = t3446 * t3453 * t9063;
    (t43854, t43875, t43878, t43887)
}
