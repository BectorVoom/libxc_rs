//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1420/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1420<F: Float>(t16974: F, t16975: F, t16976: F, t16979: F, t16980: F, t16981: F, t16983: F, t16985: F, t16988: F, t16991: F, t16993: F, t16996: F, t16998: F, t17000: F, t17003: F) -> F {
    let t18293 = t16974 - t16975 - t16976 + t16979 + t16980 + t16981 + t16983 + t16985 + t16988 + t16991 + t16993 - t16996 - t16998 + t17000 + t17003;
    t18293
}
