//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1092/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1092<F: Float>(t11941: F, t11943: F, t11945: F, t11947: F, t11949: F, t11953: F, t11955: F, t11956: F, t11957: F, t11958: F, t11960: F, t14979: F, t14980: F, t11961: F, t11962: F, t11963: F, t11964: F, t11965: F, t11966: F, t11968: F, t11973: F, t11978: F, t11982: F, t11988: F, t11992: F, t11999: F) -> (F, F) {
    let t14982 = t11941 - t11943 + t14979 + 0.09973633333333333 * t14980 - t11945 - t11947 - t11949 + t11953 - t11955 + t11956 - t11957 - t11958 + t11960;
    let t14983 = -t11961 + t11962 + t11963 - t11964 + t11965 + t11966 + t11968 + t11973 + t11978 + t11982 - t11988 + t11992 + t11999;
    (t14982, t14983)
}
