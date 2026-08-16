//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1115/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1115(t3308: f64, t574: f64, t7940: f64, t11797: f64, t1584: f64, t10776: f64, t7442: f64, t10772: f64, t7449: f64, t10698: f64, t2559: f64, t3295: f64, t7934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39385 = t574 * t3308 * t7940;
    let t39387 = t1584 * t11797;
    let t39390 = t10776 * t3308 * t7442;
    let t39393 = t10772 * t3308 * t7449;
    let t39395 = t10698 * t2559;
    let t39396 = 0.12805040077930161442e0_f64 * t39395;
    let t39397 = t3295 * t7934;
    (t39385, t39387, t39390, t39393, t39396, t39397)
}
