//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 945/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk945<F: Float>(t25777: F, t334: F, t5770: F, t6557: F, t45: F, t7796: F, t1201: F, t1213: F, t19706: F, t2107: F, t25665: F, t25667: F, t25669: F, t25672: F, t25679: F, t25683: F, t25685: F, t25687: F, t25760: F, t25762: F, t5765: F, t5771: F, t5790: F, t5795: F) -> (F, F, F) {
    let t25778 = t25777 * t334;
    let t25781 = t5770 * t6557;
    let t25786 = t45 * t7796;
    let t25791 = t25665 + t25667 - 0.17315755899375863299e2 * t1201 * t25669 - 0.34631511798751726598e2 * t1201 * t25672 + 0.23392893589820816284e1 * t5765 * t5771 - 0.346315117987517266e2 * t5765 * t5795 - 0.35089340384731224426e1 * t1201 * t25679 + t25683 - t25685 + t25687 + t25760 + t25762 + 0.19751789702565206229e-1 * t45 * t25778 + 0.23392893589820816284e1 * t1201 * t25781 - 0.11696446794910408142e1 * t5765 * t5790 - 0.58482233974552040708e0 * t25786 * t1213 - 0.11696446794910408142e1 * t19706 * t2107;
    (t25778, t25781, t25791)
}
