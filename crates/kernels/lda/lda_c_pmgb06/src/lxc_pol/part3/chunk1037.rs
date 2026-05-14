//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1037/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1037<F: Float>(t11915: F, t11918: F, t11934: F, t11937: F, t11940: F, t11943: F, t11946: F, t11951: F, t11953: F, t11955: F, t11959: F, t11970: F, t11972: F, t11974: F, t11977: F, t11981: F, t11985: F, t11987: F, t11990: F, t11994: F, t12000: F, t12003: F, t12005: F) -> (F, F) {
    let t14331 = -t11915 - t11918 + t11934 - t11937 - t11940 - t11943 - t11946 - t11951 - t11953 + t11955 + t11959 - t11970;
    let t14335 = -t11972 + t11974 + t11977 - t11981 - t11985 + t11987 + t11990 + t11994 - t12000 + t12003 - t12005;
    (t14331, t14335)
}
