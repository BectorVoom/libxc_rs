//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 818/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk818<F: Float>(t405: F, t9118: F, t2765: F, t2777: F, t411: F, t3357: F, t1729: F, t2763: F, t1664: F, t440: F, t164: F, t8756: F, t4137: F, t479: F, t8832: F, t4107: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9164 = t405 * t9118;
    let t9166 = t2765 * t2777 * t411;
    let t9169 = t2765 * t3357;
    let t9172 = t1729 * t2763;
    let t9174 = t2765 * t1664 * t440;
    let t9178 = 0.0014238371845981686 * t8756 * t164;
    let t9180 = 0.0004746123948660562 * t4137 * t479;
    let t9181 = t8832 * t164;
    let t9186 = t4107 * t479;
    (t9164, t9166, t9169, t9172, t9174, t9178, t9180, t9181, t9186)
}
