//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1126/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1126<F: Float>(t21089: F, t21091: F, t21093: F, t21096: F, t21099: F, t21104: F, t21106: F, t21107: F, t21108: F, t21109: F, t21112: F, t21113: F, t21114: F, t16529: F, t21115: F, t21116: F, t21118: F, t21119: F, t21120: F, t21121: F, t21123: F, t21124: F, t21128: F, t21129: F, t21130: F, t21131: F) -> (F, F) {
    let t23215 = t21089 + t21091 + t21093 - t21096 - t21099 + t21104 - t21106 - t21107 + t21108 - t21109 + t21112 + t21113 + t21114;
    let t23218 = -t21115 + t21116 + t21118 - t21119 - t21120 + 4.0 * t16529 + t21121 - t21123 + t21124 + t21128 + t21129 + t21130 - t21131;
    (t23215, t23218)
}
