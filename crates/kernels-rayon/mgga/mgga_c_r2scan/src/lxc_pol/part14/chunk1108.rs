//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1108/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1108(t481: f64, t9577: f64, t792: f64, t1234: f64, t3574: f64, t2259: f64, t2867: f64, t10943: f64, t11603: f64, t2333: f64, t2847: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39264 = t9577 * t481;
    let t39268 = t9577 * t792;
    let t39279 = t3574 * t1234;
    let t39286 = t2867 * t2259;
    let t39290 = t10943 * t11603;
    let t39299 = t2333 * t2847;
    let t39300 = t39299 * t795;
    (t39264, t39268, t39279, t39286, t39290, t39300)
}
