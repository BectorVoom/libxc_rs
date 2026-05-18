//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1058/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1058<F: Float>(t14484: F, t30153: F, t4271: F, t1471: F, t21230: F, t7706: F, t196: F, t30738: F, t6298: F, t7710: F, t1472: F, t30158: F) -> (F, F, F, F, F) {
    let t31332 = t4271 * t14484 * t30153;
    let t31336 = t1471 * t21230 * t7706;
    let t31339 = t30738 * t196;
    let t31343 = t1471 * t6298 * t7710;
    let t31347 = t1471 * t1472 * t30158;
    (t31332, t31336, t31339, t31343, t31347)
}
