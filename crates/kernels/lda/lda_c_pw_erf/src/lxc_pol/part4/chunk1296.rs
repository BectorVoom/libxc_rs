//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1296/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1296<F: Float>(t16567: F, t16569: F, t16572: F, t16577: F, t16581: F, t16584: F, t16586: F, t16588: F, t16590: F, t16594: F, t16598: F, t16601: F, t16605: F, t16609: F, t16611: F, t16615: F, t16619: F) -> (F,) {
    let t19184 = -t16567 + t16569 + t16572 + t16577 - t16581 - t16584 - t16586 - t16588 + t16590 + t16594 + t16598 + t16601 - t16605 + t16609 - t16611 - t16615 + t16619;
    (t19184,)
}
