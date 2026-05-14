//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1101/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1101<F: Float>(t12617: F, t12622: F, t12624: F, t12626: F, t12628: F, t12630: F, t12632: F, t12634: F, t12636: F, t12638: F, t12640: F, t12643: F, t12645: F, t12650: F, t12653: F, t12655: F, t12659: F, t12662: F, t12664: F, t12666: F, t12668: F, t12669: F, t12671: F, t12673: F, t12674: F, t12676: F) -> (F, F) {
    let t15025 = t12617 - t12622 - t12624 - t12626 - t12628 + t12630 + t12632 - t12634 + t12636 - t12638 - t12640 - t12643 + t12645;
    let t15026 = t12650 + t12653 - t12655 + t12659 + t12662 - t12664 + t12666 + t12668 - t12669 - t12671 - t12673 + t12674 + t12676;
    (t15025, t15026)
}
