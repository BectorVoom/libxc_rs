//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 808/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk808<F: Float>(t1776: F, t28312: F, t1775: F, t10833: F, t28368: F, t10832: F, t7262: F, t8820: F, t7261: F, t2364: F, t5015: F, t28385: F, t7242: F, t2464: F, t8510: F, t10802: F) -> (F, F, F, F, F, F) {
    let t29010 = t1776 * t28312;
    let t29011 = t1775 * t29010;
    let t29016 = t10833 * t28368;
    let t29017 = t10832 * t29016;
    let t29024 = t7262 * t8820;
    let t29025 = t7261 * t29024;
    let t29028 = t2364 * t8820;
    let t29029 = t5015 * t29028;
    let t29032 = t7242 * t28385;
    let t29035 = t8510 * t2464;
    let t29036 = t10802 * t29035;
    (t29011, t29017, t29025, t29029, t29032, t29036)
}
