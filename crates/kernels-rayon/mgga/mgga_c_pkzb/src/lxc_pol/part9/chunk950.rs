//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 950/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk950(t218: f64, t2774: f64, t675: f64, t2778: f64, t1066: f64, t1843: f64, t219: f64, t2739: f64, t655: f64, t208: f64, t7350: f64, t5525: f64, t5560: f64, t5563: f64, t5566: f64, t5852: f64, t5859: f64, t7357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7386 = t218 * t675 * t2774;
    let t7387 = 0.41678e0_f64 * t7386;
    let t7389 = t218 * t675 * t2778;
    let t7390 = 0.41678e0_f64 * t7389;
    let t7391 = t1843 * t1066;
    let t7393 = t218 * t219 * t7391;
    let t7395 = t655 * t2739;
    let t7397 = t218 * t219 * t7395;
    let t7399 = t208 * t7350;
    let t7401 = t218 * t219 * t7399;
    let t7406 = -0.516475e0_f64 * t5525 + 0.68863333333333333333e0_f64 * t7357 - t7387 - t7390 + 0.312585e0_f64 * t7393 + 0.62517e0_f64 * t7397 + 0.312585e0_f64 * t7401 - t5852 - t5859 + 0.69463333333333333333e0_f64 * t5560 - 0.20839e0_f64 * t5563 - 0.20839e0_f64 * t5566;
    (t7386, t7389, t7391, t7393, t7395, t7397, t7399, t7401, t7406)
}
