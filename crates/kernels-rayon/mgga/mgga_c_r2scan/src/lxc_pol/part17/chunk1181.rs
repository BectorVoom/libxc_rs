//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1181/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1181(t38145: f64, t6085: f64, t9242: f64, t6093: f64, t9246: f64, t2201: f64, t3216: f64, t3319: f64, t3320: f64, t10698: f64, t12523: f64, t3602: f64, t39922: f64, t8081: f64) -> (f64, f64, f64, f64, f64) {
    let t43441 = t6085 * t38145 * t9242;
    let t43447 = t6093 * t38145 * t9246;
    let t43451 = t2201 * t3319 * t3320 * t3216;
    let t43454 = t10698 * t12523;
    let t43459 = t39922 * t3602 * t8081;
    (t43441, t43447, t43451, t43454, t43459)
}
