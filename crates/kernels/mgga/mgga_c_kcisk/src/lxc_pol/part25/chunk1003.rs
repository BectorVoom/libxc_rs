//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1003/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1003<F: Float>(t15914: F, t16590: F, t16642: F, t16699: F, t17013: F, t17071: F, t17131: F, t17767: F, t752: F, t1907: F, t7291: F, t1957: F, t2541: F, t5217: F, t5219: F, t5339: F, t7293: F) -> (F, F, F, F, F, F, F) {
    let t17770 = t15914 + t16590 + t16642 + t16699 + t17013 + t17071 + t17131 + t17767;
    let t17771 = t17770 * t752;
    let t17772 = t7291 * t1907;
    let t17774 = 2.0 * t17772 * t1957;
    let t17775 = t2541 * t5217;
    let t17777 = 2.0 * t17775 * t5219;
    let t17778 = t7293 * t5339;
    (t17770, t17771, t17772, t17774, t17775, t17777, t17778)
}
