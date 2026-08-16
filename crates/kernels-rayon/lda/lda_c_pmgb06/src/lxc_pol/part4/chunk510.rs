//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 510/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk510(t161: f64, t2018: f64, t1798: f64, t205: f64, t208: f64, t579: f64, t794: f64, t213: f64, t871: f64, t97: f64, t588: f64, t591: f64, t872: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2019 = t161 * t2018;
    let t2020 = t2019 / 45.0_f64;
    let t2021 = t1798 * t205;
    let t2022 = t2021 * t208;
    let t2025 = t794 * t579;
    let t2026 = t2025 * t208;
    let t2027 = t2026 * t213;
    let t2029 = t871 * t97;
    let t2030 = t2029 * t588;
    let t2032 = t872 * t591;
    (t2019, t2020, t2021, t2022, t2025, t2026, t2027, t2029, t2030, t2032)
}
