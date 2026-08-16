//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1060/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1060(t260: f64, t6100: f64, t1102: f64, t1104: f64, t3314: f64, t875: f64, t10648: f64, t10651: f64, t10972: f64, t37373: f64, t37369: f64, t10977: f64, t10981: f64, t37372: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37449 = t260 * t6100;
    let t37451 = t1102 * t37449 * t1104;
    let t37453 = t3314 * t875;
    let t37455 = t10648 * t37453 * t10651;
    let t37458 = t37373 * t10972;
    let t37460 = t37369 * t10972;
    let t37463 = t37372 * t10977 * t10981;
    (t37449, t37451, t37453, t37455, t37458, t37460, t37463)
}
