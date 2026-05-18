//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 632/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk632<F: Float>(t5556: F, t567: F, t564: F, t1390: F, t470: F, t3529: F, t453: F, t1336: F, t140: F, t3532: F, t5: F, t969: F) -> (F, F, F, F, F) {
    let t5557 = t567 * t5556;
    let t5558 = t564 * t5557;
    let t5625 = t470 * t1390;
    let t5631 = t3529 * t453;
    let t5633 = t140 * t1336 * t5631;
    let t5634 = t470 * t3532;
    let t5680 = t5 * t969;
    (t5558, t5625, t5633, t5634, t5680)
}
