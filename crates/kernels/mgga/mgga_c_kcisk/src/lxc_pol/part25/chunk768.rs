//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 768/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk768<F: Float>(t20: F, t2447: F, t654: F, t1693: F, t2454: F, t648: F, t2364: F, t9665: F, t1775: F, t2464: F, t9670: F, t7261: F, t2063: F, t5185: F, t9679: F, t1799: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9926 = t2447 * t654 * t20;
    let t9927 = t1693 * t9926;
    let t9931 = t648 * t2454 * t20;
    let t9932 = t1693 * t9931;
    let t9935 = t9665 * t2364;
    let t9936 = t1775 * t9935;
    let t9939 = t9670 * t2464;
    let t9940 = t7261 * t9939;
    let t9945 = t5185 * t2063;
    let t9946 = t9679 * t9945;
    let t9947 = t1799 * t9946;
    (t9926, t9927, t9931, t9932, t9935, t9936, t9939, t9940, t9945, t9946, t9947)
}
