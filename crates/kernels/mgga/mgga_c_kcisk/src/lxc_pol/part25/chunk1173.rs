//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1173/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1173<F: Float>(t7444: F, t9696: F, t17772: F, t2799: F, t17775: F, t9699: F, t7293: F, t9718: F, t11694: F, t9967: F, t1957: F, t11701: F, t2594: F, t5218: F, t5213: F, t9988: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34293 = t9696 * t7444;
    let t34294 = t17772 * t2799;
    let t34296 = 2.0 * t17775 * t9699;
    let t34297 = t7293 * t9718;
    let t34299 = 2.0 * t11694 * t9967;
    let t34300 = t9967 * t1957;
    let t34302 = 6.0 * t11701 * t34300;
    let t34303 = t9718 * t2594;
    let t34305 = 2.0 * t5218 * t34303;
    let t34306 = t2799 * t7444;
    let t34308 = 2.0 * t5218 * t34306;
    let t34309 = t5213 * t9988;
    (t34293, t34294, t34296, t34297, t34299, t34300, t34302, t34303, t34305, t34306, t34308, t34309)
}
