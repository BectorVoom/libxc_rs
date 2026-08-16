//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 958/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk958(t1266: f64, t321: f64, t502: f64, t818: f64, t826: f64, t1275: f64, t263: f64, t1277: f64, t1289: f64, t3358: f64, t1070: f64, t6651: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11031 = t1266 * t321;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    let t11035 = 2.0_f64 / 3.0_f64 * t11034;
    let t11036 = t263 * t1275;
    let t11037 = t11036 * t1277;
    let t11039 = t3358 * t1289;
    let t11041 = t6651 * t1070;
    (t11031, t11033, t11034, t11035, t11036, t11037, t11039, t11041)
}
