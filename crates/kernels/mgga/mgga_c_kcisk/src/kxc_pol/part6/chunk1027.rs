//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1027/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1027<F: Float>(t30801: F, t30836: F, t30855: F, t30873: F, t30153: F, t425: F, t2191: F, t25947: F, t2181: F, t7710: F, t25921: F, t5646: F, t7897: F) -> (F, F, F, F, F, F) {
    let t30875 = t30801 + t30836 + t30855 + t30873;
    let t30877 = t425 * t30153;
    let t30880 = t25947 * t2191;
    let t30883 = t2181 * t7710;
    let t30886 = t25921 * t2191;
    let t30889 = t5646 * t7897;
    (t30875, t30877, t30880, t30883, t30886, t30889)
}
