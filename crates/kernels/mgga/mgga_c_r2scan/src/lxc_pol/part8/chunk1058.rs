//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1058/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1058<F: Float>(t3245: F, t986: F, t106: F, t6897: F, t97: F, t288: F, t9937: F, t1048: F, t8601: F, t910: F, t9788: F, t2858: F, t4873: F, t5039: F, t6881: F, t7129: F, t9804: F, t9816: F, t9917: F, t9918: F, t9919: F, t9920: F, t9921: F, t9922: F, t9923: F, t9925: F) -> (F, F, F, F, F, F, F) {
    let t10572 = t3245 * t986;
    let t10575 = t97 * t106 * t10572 * t6897;
    let t10576 = 2.0 * t10575;
    let t10580 = t97 * t9937 * t288;
    let t10581 = 6.0 * t10580;
    let t10583 = t1048 * t8601 * t986;
    let t10584 = 3.0 * t10583;
    let t10585 = t9788 * t910;
    let t10586 = t2858 * t10585;
    let t10587 = 18.0 * t10586;
    let t10588 = -0.7089e1 * t9804 - t9917 - t9918 - t10576 + t9919 + t9920 + t6881 - t9921 - 0.7089e1 * t7129 - 0.14178e2 * t9816 - t10581 - t4873 + t10584 - t10587 + t9922 + t9923 - t5039 - t9925;
    (t10572, t10576, t10581, t10584, t10585, t10587, t10588)
}
