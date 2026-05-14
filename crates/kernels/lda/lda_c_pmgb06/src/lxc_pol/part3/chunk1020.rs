//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1020<F: Float>(t1385: F, t3010: F, t439: F, t5271: F, t13958: F, t13960: F, t13963: F, t13968: F, t13970: F, t13972: F, t13974: F, t13978: F, t13983: F, t13985: F, t13988: F, t1444: F, t5477: F) -> (F, F, F) {
    let t13992 = 2.0 / 15.0 * t439 * t1385 * t5271 * t3010;
    let t13993 = -t13958 - t13960 + t13963 + t13968 + t13970 - t13972 - t13974 + t13978 - t13983 + t13985 - t13988 - t13992;
    let t13995 = 2.0 / 15.0 * t1444 * t5477;
    (t13992, t13993, t13995)
}
