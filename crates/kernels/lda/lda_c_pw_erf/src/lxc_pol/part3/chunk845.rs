//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 845/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk845<F: Float>(t5429: F, t5803: F, t5804: F, t5838: F, t5839: F, t5848: F, t5849: F, t5851: F, t5853: F, t5857: F, t5861: F, t5863: F, t5864: F, t5867: F, t5868: F, t5876: F) -> F {
    let t5880 = t5803 + t5804 + t5838 + t5839 + t5848 + t5849 + t5851 + t5853 + t5857 + t5861 + t5863 + t5864 + t5867 + t5868 + t5876 + t5429;
    t5880
}
