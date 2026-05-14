//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 818/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk818<F: Float>(t5111: F, t960: F, t5114: F, t965: F, t5117: F, t970: F, t1857: F, t3123: F, t5144: F, t5147: F, t5152: F, t5155: F, t5160: F, t5163: F, t1060: F, t1846: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11537 = t960 * t5111;
    let t11542 = t965 * t5114;
    let t11548 = t970 * t5117;
    let t11562 = t3123 * t1857;
    let t11564 = t970 * t5144;
    let t11566 = t970 * t5147;
    let t11574 = t960 * t5152;
    let t11578 = t960 * t5155;
    let t11586 = t965 * t5160;
    let t11588 = t965 * t5163;
    let t11605 = t1846 * t1060;
    (t11537, t11542, t11548, t11562, t11564, t11566, t11574, t11578, t11586, t11588, t11605)
}
