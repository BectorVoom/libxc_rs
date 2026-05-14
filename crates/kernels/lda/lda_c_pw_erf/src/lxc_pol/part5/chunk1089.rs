//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1089/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1089<F: Float>(t10015: F, t7749: F, t3965: F, t4479: F, t6460: F, t12475: F, t6464: F, t12143: F, t7752: F, t10027: F, t3974: F, t4475: F, t6396: F, t7016: F, t795: F, t185: F, t514: F, t7793: F) -> (F, F, F, F, F, F, F, F) {
    let t22616 = 16.0 / 15.0 * t10015 * t7749;
    let t22619 = 16.0 / 15.0 * t3965 * t4479 * t6460;
    let t22622 = 32.0 / 15.0 * t12475 * t4479 * t6464;
    let t22624 = 16.0 / 15.0 * t12143 * t7752;
    let t22626 = 16.0 / 15.0 * t10027 * t7752;
    let t22629 = 16.0 / 15.0 * t3974 * t4475 * t6396;
    let t22630 = t795 * t7016;
    let t22631 = 4.0 / 15.0 * t22630;
    let t22633 = t185 * t514 * t7793;
    (t22616, t22619, t22622, t22624, t22626, t22629, t22631, t22633)
}
