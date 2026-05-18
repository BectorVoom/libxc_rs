//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 788/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk788<F: Float>(t2250: F, t979: F, t1390: F, t2209: F, t3532: F, t2242: F, t306: F, t140: F, t2253: F, t430: F, t2257: F, t3783: F) -> (F, F, F, F, F, F) {
    let t21152 = t979 * t2250;
    let t21230 = t2209 * t1390;
    let t21239 = t2209 * t3532;
    let t21252 = t2242 * t306;
    let t21256 = t140 * t430 * t2253;
    let t21314 = t2257 * t3783;
    (t21152, t21230, t21239, t21252, t21256, t21314)
}
