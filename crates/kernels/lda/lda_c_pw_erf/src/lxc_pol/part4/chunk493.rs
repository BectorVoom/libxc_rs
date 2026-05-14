//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 493/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk493<F: Float>(t1308: F, t2022: F, t571: F, t494: F, t789: F, t1326: F, t1325: F, t542: F) -> (F, F, F, F, F, F) {
    let t2023 = t1308 * t2022;
    let t2025 = 4.0 / 45.0 * t571 * t2023;
    let t2026 = t789 * t494;
    let t2027 = t1326 * t2026;
    let t2029 = 8.0 / 45.0 * t1325 * t2027;
    let t2030 = t789 * t542;
    (t2023, t2025, t2026, t2027, t2029, t2030)
}
