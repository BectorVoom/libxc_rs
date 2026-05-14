//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 885/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk885<F: Float>(t1704: F, t2765: F, t756: F, t343: F, t780: F, t159: F, t285: F, t4437: F, t477: F, t10847: F, t10852: F, t1726: F, t1729: F, t2764: F, t4430: F, t454: F, t5925: F, t777: F, t8768: F, t8822: F, t8825: F, t8827: F, t8831: F, t8834: F, t8838: F, t8842: F, t8845: F) -> (F, F) {
    let t11543 = t2765 * t756 * t1704;
    let t11546 = t343 * t780;
    let t11548 = t11546 * t159 * t285;
    let t11551 = t4437 * t477 * t285;
    let t11556 = 0.585406996056892 * t8822 + t8825 + 0.012203831437512505 * t8827 + t8831 - 0.020146007452401596 * t8834 - t8838 + t8842 + 0.004067943812504169 * t8845 - 6.0 * t8768 * t4430 + 18.0 * t1729 * t1726 * t454 * t5925 - 3.0 * t2764 * t11543 - 0.006715335817467199 * t11548 + 0.004067943812504169 * t11551 - t777 * t10847 - 3.0 * t777 * t10852;
    (t11546, t11556)
}
