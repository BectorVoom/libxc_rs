//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 490/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk490<F: Float>(t1976: F, t503: F, t1953: F, t1241: F, t1501: F, t1964: F, t1969: F, t1974: F, t173: F, t184: F) -> (F, F, F, F, F) {
    let t1977 = t503 * t1976;
    let t1978 = t1953 * t1977;
    let t1980 = -t1501 - F::new(0.0006297222222222223) * t1241 - F::new(0.0006297222222222223) * t1964 + F::new(0.0012594444444444445) * t1969 - F::new(0.003778333333333333) * t1974 + F::new(0.003778333333333333) * t1978;
    let t1981 = t173 * t1980;
    let t1982 = t1981 * t184;
    (t1977, t1978, t1980, t1981, t1982)
}
