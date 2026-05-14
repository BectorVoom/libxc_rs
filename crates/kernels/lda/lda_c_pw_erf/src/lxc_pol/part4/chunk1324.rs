//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1324/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1324<F: Float>(t17900: F, t17902: F, t17904: F, t17907: F, t17911: F, t17912: F, t17913: F, t17914: F, t17915: F, t17916: F, t17917: F, t17918: F, t17919: F, t17920: F, t17921: F, t17923: F, t17925: F) -> (F,) {
    let t19280 = -t17900 + t17902 + t17904 - t17907 + t17911 - t17912 - t17913 - t17914 - t17915 + t17916 - t17917 - t17918 - t17919 + t17920 - t17921 + t17923 + t17925;
    (t19280,)
}
