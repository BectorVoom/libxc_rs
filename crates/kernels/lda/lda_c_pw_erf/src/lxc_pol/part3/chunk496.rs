//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 496/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk496<F: Float>(t1944: F, t2017: F, t571: F, t558: F, t833: F, t352: F, t1308: F, t494: F, t789: F, t1326: F, t1325: F, t542: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2018 = t2017 * t1944;
    let t2020 = F::new(4.0) / F::new(27.0) * t571 * t2018;
    let t2021 = t833 * t558;
    let t2022 = t2021 * t352;
    let t2023 = t1308 * t2022;
    let t2025 = F::new(4.0) / F::new(45.0) * t571 * t2023;
    let t2026 = t789 * t494;
    let t2027 = t1326 * t2026;
    let t2029 = F::new(8.0) / F::new(45.0) * t1325 * t2027;
    let t2030 = t789 * t542;
    (t2018, t2020, t2021, t2022, t2023, t2025, t2026, t2027, t2029, t2030)
}
