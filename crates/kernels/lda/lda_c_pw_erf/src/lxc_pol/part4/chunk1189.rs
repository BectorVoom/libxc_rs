//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1189/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1189<F: Float>(t13048: F, t13051: F, t13054: F, t13066: F, t13068: F, t13070: F, t13073: F, t13078: F, t13082: F, t13097: F, t13099: F, t13102: F, t13105: F, t13128: F, t13130: F, t3787: F, t519: F, t6908: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17563 = 32.0 / 405.0 * t13048;
    let t17564 = 64.0 / 405.0 * t13051;
    let t17565 = 16.0 / 135.0 * t13054;
    let t17566 = 16.0 / 81.0 * t13066;
    let t17567 = 64.0 / 135.0 * t13068;
    let t17568 = 64.0 / 135.0 * t13070;
    let t17569 = 32.0 / 135.0 * t13073;
    let t17570 = 32.0 / 81.0 * t13078;
    let t17571 = 64.0 / 135.0 * t13082;
    let t17572 = 64.0 / 135.0 * t13097;
    let t17573 = 64.0 / 135.0 * t13099;
    let t17574 = 32.0 / 135.0 * t13102;
    let t17575 = 32.0 / 81.0 * t13105;
    let t17576 = 64.0 / 135.0 * t13128;
    let t17577 = 128.0 / 135.0 * t13130;
    let t17579 = t519 * t3787 * t6908;
    (t17563, t17564, t17565, t17566, t17567, t17568, t17569, t17570, t17571, t17572, t17573, t17574, t17575, t17576, t17577, t17579)
}
