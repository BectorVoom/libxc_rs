//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1194/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1194<F: Float>(t11972: F, t11974: F, t11977: F, t11981: F, t11985: F, t11987: F, t11990: F, t11994: F, t12000: F, t12003: F, t12005: F, t12011: F, t12015: F, t12017: F, t12019: F, t12021: F, t12023: F, t12026: F, t12028: F, t12032: F, t12035: F, t12038: F, t12040: F) -> (F, F) {
    let t14335 = -t11972 + t11974 + t11977 - t11981 - t11985 + t11987 + t11990 + t11994 - t12000 + t12003 - t12005;
    let t14336 = -t12011 - t12015 - t12017 - t12019 - t12021 + t12023 + t12026 + t12028 + t12032 + t12035 - t12038 - t12040;
    (t14335, t14336)
}
