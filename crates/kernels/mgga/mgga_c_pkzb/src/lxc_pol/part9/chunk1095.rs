//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1095/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1095<F: Float>(t218: F, t675: F, t7391: F, t7395: F, t7399: F, t1066: F, t219: F, t5537: F, t1843: F, t2739: F, t655: F, t7350: F, t20743: F, t208: F, t20716: F, t17351: F, t17354: F, t17357: F, t17455: F, t20705: F, t20719: F, t20745: F) -> (F, F, F, F, F, F, F, F) {
    let t20759 = t218 * t675 * t7391;
    let t20762 = t218 * t675 * t7395;
    let t20765 = t218 * t675 * t7399;
    let t20769 = t218 * t219 * t5537 * t1066;
    let t20773 = t218 * t219 * t1843 * t2739;
    let t20777 = t218 * t219 * t655 * t7350;
    let t20781 = t218 * t219 * t208 * t20743;
    let t20787 = 4.0 / 3.0 * t20716;
    let t20788 = t17455 - 28.0 / 9.0 * t17351 + 4.0 / 3.0 * t17354 - t17357 / 3.0 - 28.0 / 27.0 * t20705 + t20787 - t20719 + t20745;
    (t20759, t20762, t20765, t20769, t20773, t20777, t20781, t20788)
}
