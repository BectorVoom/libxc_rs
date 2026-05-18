//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 756/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk756<F: Float>(t4811: F, t4818: F, t4817: F, t5069: F, t1869: F, t1894: F, t4797: F, t1801: F, t5062: F, t1755: F, t695: F, t1060: F, t4972: F) -> (F, F, F, F, F, F) {
    let t11663 = t4811 * t4818;
    let t11668 = t4817 * t5069;
    let t11669 = t1869 * t11668;
    let t11671 = t4797 * t1894;
    let t11672 = t1801 * t11671;
    let t11673 = t5062 * t11672;
    let t11674 = t1869 * t11673;
    let t11676 = t1755 * t695;
    let t11677 = t1060 * t4972;
    (t11663, t11669, t11671, t11674, t11676, t11677)
}
