//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 508/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk508<F: Float>(t2178: F, t3748: F, t1390: F, t470: F, t3529: F, t453: F, t1336: F, t140: F, t3532: F, t2181: F, t443: F, t1354: F, t2059: F) -> (F, F, F, F, F, F) {
    let t5610 = t3748 * t2178;
    let t5625 = t470 * t1390;
    let t5631 = t3529 * t453;
    let t5633 = t140 * t1336 * t5631;
    let t5634 = t470 * t3532;
    let t5641 = t443 * t2181;
    let t5646 = t1354 * t2059;
    (t5610, t5625, t5633, t5634, t5641, t5646)
}
