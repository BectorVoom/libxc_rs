//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 432/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk432(t1308: f64, t2022: f64, t571: f64, t494: f64, t789: f64) -> (f64, f64, f64) {
    let t2023 = t1308 * t2022;
    let t2025 = 4.0_f64 / 45.0_f64 * t571 * t2023;
    let t2026 = t789 * t494;
    (t2023, t2025, t2026)
}
