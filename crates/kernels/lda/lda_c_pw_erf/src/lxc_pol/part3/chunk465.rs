//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 465/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk465<F: Float>(t1954: F, t557: F, t1953: F, t1347: F, t1491: F, t1941: F, t1946: F, t1951: F, t203: F, t184: F) -> (F, F, F, F, F) {
    let t1955 = t557 * t1954;
    let t1956 = t1953 * t1955;
    let t1958 = -t1491 - 0.0006297222222222223 * t1347 - 0.0006297222222222223 * t1941 + 0.0012594444444444445 * t1946 - 0.003778333333333333 * t1951 - 0.003778333333333333 * t1956;
    let t1959 = t203 * t1958;
    let t1960 = t1959 * t184;
    (t1955, t1956, t1958, t1959, t1960)
}
