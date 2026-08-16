//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 948/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk948(t1102: f64, t3314: f64, t3457: f64, t2304: f64, t875: f64, t3434: f64, t3439: f64, t1266: f64, t321: f64, t502: f64, t818: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11008 = t1102 * t3314 * t3457;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11031 = t1266 * t321;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    (t11008, t11015, t11017, t11031, t11033, t11034)
}
