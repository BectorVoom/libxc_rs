//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1239/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1239(t322: f64, t40814: f64, t40850: f64, t1013: f64, t11063: f64, t11897: f64, t11909: f64, t11912: f64, t1292: f64, t1295: f64, t19203: f64, t2394: f64, t327: f64, t3373: f64, t3638: f64, t37015: f64, t40770: f64, t6693: f64, t829: f64, t834: f64, t8398: f64) -> (f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t40851 = t40814 + t40850;
    let t40852 = piecewise3(t324, 0.0_f64, t40851);
    let t40869 = -0.128e1_f64 * t11897 * t1292 - 0.384e1_f64 * t40770 * t1295 - 0.128e1_f64 * t37015 * t1013 - 0.256e1_f64 * t11063 * t2394 - 0.128e1_f64 * t3373 * t8398 - 0.64e0_f64 * t834 * t40852 - 0.768e1_f64 * t6693 * t11909 * t829 - 0.768e1_f64 * t6693 * t11912 * t829 - 0.384e1_f64 * t6693 * t3638 * t1292 - 0.1536e2_f64 * t19203 * t3638 * t1295 - 0.64e0_f64 * t40852 * t327;
    (t40851, t40869)
}
