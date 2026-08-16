//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1162/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1162(t10760: f64, t29283: f64, t6535: f64, t11793: f64, t2201: f64, t3613: f64, t12448: f64, t3336: f64, t1058: f64, t1060: f64, t9365: f64, t11760: f64, t11764: f64, t2207: f64) -> (f64, f64, f64, f64, f64) {
    let t43120 = t6535 * t10760 * t29283;
    let t43123 = t2201 * t3613 * t11793;
    let t43126 = t2201 * t3336 * t12448;
    let t43130 = t2201 * t1058 * t1060 * t9365;
    let t43133 = t2207 * t11760 * t11764;
    (t43120, t43123, t43126, t43130, t43133)
}
