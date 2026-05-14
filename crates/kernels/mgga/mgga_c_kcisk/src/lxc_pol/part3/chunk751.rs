//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 751/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk751<F: Float>(t12697: F, t205: F, t12699: F, t207: F, t1050: F, t3139: F, t3138: F, t12499: F, t151: F, t12435: F, t3086: F, t3107: F, t955: F, t3216: F, t196: F, t967: F) -> (F, F, F, F, F, F, F) {
    let t12712 = t205 * t12697;
    let t12713 = t207 * t12699;
    let t12714 = t12712 * t12713;
    let t12716 = t1050 * t3139;
    let t12717 = t3138 * t12716;
    let t12723 = t151 * t12499;
    let t12730 = t3086 * t12435;
    let t12734 = t955 * t3107;
    let t12735 = t3216 * t12734;
    let t12741 = t196 * t967;
    (t12714, t12717, t12723, t12730, t12734, t12735, t12741)
}
