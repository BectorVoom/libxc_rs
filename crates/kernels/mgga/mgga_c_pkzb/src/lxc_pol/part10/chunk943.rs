//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 943/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk943<F: Float>(t7386: F, t218: F, t2778: F, t675: F, t1066: F, t1843: F, t219: F, t2739: F, t655: F, t208: F, t7350: F, t5525: F, t5560: F, t5563: F, t5566: F, t5852: F, t5859: F, t7357: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7387 = 0.41678e0 * t7386;
    let t7389 = t218 * t675 * t2778;
    let t7390 = 0.41678e0 * t7389;
    let t7391 = t1843 * t1066;
    let t7393 = t218 * t219 * t7391;
    let t7395 = t655 * t2739;
    let t7397 = t218 * t219 * t7395;
    let t7399 = t208 * t7350;
    let t7401 = t218 * t219 * t7399;
    let t7406 = -0.516475e0 * t5525 + 0.68863333333333333333e0 * t7357 - t7387 - t7390 + 0.312585e0 * t7393 + 0.62517e0 * t7397 + 0.312585e0 * t7401 - t5852 - t5859 + 0.69463333333333333333e0 * t5560 - 0.20839e0 * t5563 - 0.20839e0 * t5566;
    (t7387, t7389, t7390, t7391, t7393, t7395, t7397, t7399, t7401, t7406)
}
