//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 496/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk496(t1944: f64, t2017: f64, t571: f64, t558: f64, t833: f64, t352: f64, t1308: f64, t494: f64, t789: f64, t1326: f64, t1325: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2018 = t2017 * t1944;
    let t2020 = 4.0_f64 / 27.0_f64 * t571 * t2018;
    let t2021 = t833 * t558;
    let t2022 = t2021 * t352;
    let t2023 = t1308 * t2022;
    let t2025 = 4.0_f64 / 45.0_f64 * t571 * t2023;
    let t2026 = t789 * t494;
    let t2027 = t1326 * t2026;
    let t2029 = 8.0_f64 / 45.0_f64 * t1325 * t2027;
    let t2030 = t789 * t542;
    (t2018, t2020, t2021, t2022, t2023, t2025, t2026, t2027, t2029, t2030)
}
