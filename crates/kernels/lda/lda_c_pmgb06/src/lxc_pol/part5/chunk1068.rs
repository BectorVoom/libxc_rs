//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1068/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1068<F: Float>(t19998: F, t20001: F, t20009: F, t20012: F, t20014: F, t20017: F, t20021: F, t20025: F, t20028: F, t20031: F, t20035: F, t20039: F, t12650: F, t20071: F, t20074: F, t20076: F, t20079: F, t20081: F, t20084: F, t20086: F, t20089: F, t20090: F, t20104: F, t20107: F) -> (F, F) {
    let t21949 = -t19998 - t20001 - t20009 - t20012 - t20014 - t20017 - t20021 + t20025 - t20028 + t20031 - t20035 - t20039;
    let t21951 = t20071 + t20074 + t20076 + t20079 + t20081 - t12650 + t20084 + t20086 + t20089 - t20090 + t20104 + t20107;
    (t21949, t21951)
}
