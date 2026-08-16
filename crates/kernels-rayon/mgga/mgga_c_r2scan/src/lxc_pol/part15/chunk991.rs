//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 991/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk991(t11626: f64, t3275: f64, t1561: f64, t3617: f64, t3277: f64, t10918: f64, t2867: f64, t11479: f64, t3262: f64, t3264: f64, t3332: f64, t7629: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11627 = t3275 * t11626;
    let t11628 = t11627 / 2.0_f64;
    let t11629 = t1561 * t3617;
    let t11631 = t3275 * t11629 * t3277;
    let t11632 = 5.0_f64 / 16.0_f64 * t11631;
    let t11634 = t3275 * t10918 * t2867;
    let t11635 = t11634 / 4.0_f64;
    let t11637 = t3262 * t11479 * t3264;
    let t11638 = 3.0_f64 / 4.0_f64 * t11637;
    let t11640 = t3332 * t7629;
    (t11627, t11628, t11629, t11631, t11632, t11634, t11635, t11637, t11638, t11640)
}
