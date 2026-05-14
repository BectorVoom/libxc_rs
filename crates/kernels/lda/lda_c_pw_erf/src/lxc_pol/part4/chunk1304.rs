//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1304/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1304<F: Float>(t11035: F, t11038: F, t16956: F, t16958: F, t16962: F, t16964: F, t16969: F, t16973: F, t16978: F, t16983: F, t16985: F, t16987: F, t16988: F, t16990: F, t16991: F, t16992: F, t16993: F) -> (F,) {
    let t19202 = -t16956 - t16958 - t16962 + t16964 - t16969 + t16973 + 0.033245444444444446 * t11035 - t16978 - t16983 - t16985 + t16987 + t16988 - t16990 - t16991 - t11038 - t16992 - t16993;
    (t19202,)
}
