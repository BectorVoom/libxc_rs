//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 932/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk932<F: Float>(t28800: F, t7303: F, t7302: F, t2579: F, t9078: F, t1948: F, t28294: F, t5322: F, t5321: F, t28749: F, t7316: F, t7315: F) -> (F, F, F, F) {
    let t29541 = t7303 * t28800;
    let t29542 = t7302 * t29541;
    let t29544 = t9078 * t2579;
    let t29545 = t1948 * t29544;
    let t29547 = t5322 * t28294;
    let t29548 = t5321 * t29547;
    let t29550 = t7316 * t28749;
    let t29551 = t7315 * t29550;
    (t29542, t29545, t29548, t29551)
}
