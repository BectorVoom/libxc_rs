//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1033/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1033<F: Float>(t240: F, t7796: F, t1213: F, t1550: F, t2107: F, t21710: F, t25665: F, t25667: F, t25679: F, t25683: F, t25685: F, t25687: F, t25760: F, t25762: F, t25867: F, t25875: F, t4486: F, t5771: F, t5790: F, t5795: F, t6568: F, t7804: F, t7821: F) -> (F,) {
    let t27491 = t240 * t7796;
    let t27508 = -0.1025389702100779493e4 * t1550 * t25867 + t25665 + t25667 + 0.11696446794910408142e1 * t4486 * t7804 - 0.58482233974552040708e0 * t27491 * t1213 - 0.11696446794910408142e1 * t21710 * t2107 - 0.11696446794910408142e1 * t6568 * t5790 - 0.346315117987517266e2 * t6568 * t5795 - 0.35089340384731224426e1 * t1550 * t25679 - 0.58482233974552040708e0 * t4486 * t7821 + 0.23392893589820816284e1 * t6568 * t5771 + t25683 - t25685 + t25687 + t25760 + t25762 + 0.1038945353962551798e3 * t1550 * t25875;
    (t27508,)
}
