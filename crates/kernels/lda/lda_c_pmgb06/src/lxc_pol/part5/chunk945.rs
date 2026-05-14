//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 945/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk945<F: Float>(t1531: F, t7290: F, t332: F, t36: F, t453: F, t1525: F, t19490: F, t12396: F, t12406: F, t19618: F, t12397: F, t350: F, t7478: F, t7498: F, t19756: F, t19760: F, t19764: F, t19768: F, t19772: F, t19774: F, t19776: F, t19780: F, t19784: F, t19788: F, t19793: F, t19796: F, t19799: F, t9215: F) -> (F, F, F, F, F, F, F, F) {
    let t19801 = t1531 * t7290;
    let t19802 = t19801 * t332;
    let t19804 = t36 * t453 * t19802;
    let t19807 = t36 * t1525 * t19490;
    let t19811 = t12396 * t12406 * t19618;
    let t19814 = t12396 * t12397 * t19618;
    let t19816 = t350 * t7478;
    let t19818 = t350 * t7498;
    let t19820 = 0.8638 * t19756 - 0.07198333333333333 * t19760 - 0.14396666666666666 * t19764 + 0.8638 * t19768 - 1.2957 * t19772 + 0.023994444444444443 * t19774 - 0.07198333333333333 * t19776 + 0.21595 * t19780 + 0.4319 * t19784 - 0.8638 * t19788 + 0.47988888888888886 * t19793 - 0.10664197530864197 * t19796 - 0.23994444444444443 * t19799 + 0.07198333333333333 * t19804 - 0.023994444444444443 * t19807 + 0.03732469135802469 * t9215 + 0.4319 * t19811 - 0.11997222222222222 * t19814 + 0.013330246913580247 * t19816 + 0.011997222222222222 * t19818;
    (t19802, t19804, t19807, t19811, t19814, t19816, t19818, t19820)
}
