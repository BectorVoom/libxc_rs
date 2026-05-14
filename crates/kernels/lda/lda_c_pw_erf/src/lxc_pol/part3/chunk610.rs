//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 610/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk610<F: Float>(t3881: F, t1458: F, t9: F, t1461: F, t519: F, t2961: F, t523: F, t522: F, t1251: F, t187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3882 = 8.0 / 45.0 * t3881;
    let t3883 = t9 * t1458;
    let t3884 = t3883 * t1461;
    let t3885 = t519 * t3884;
    let t3886 = 8.0 / 27.0 * t3885;
    let t3887 = t523 * t2961;
    let t3888 = t522 * t3887;
    let t3890 = 4.0 / 45.0 * t519 * t3888;
    let t3892 = 1.0 / t187 / t1251;
    (t3882, t3883, t3884, t3885, t3886, t3887, t3888, t3890, t3892)
}
