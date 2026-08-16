//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 496/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk496(t2025: f64, t208: f64, t213: f64, t871: f64, t97: f64, t588: f64, t591: f64, t872: f64, t1424: f64, t1448: f64, t1505: f64, t1518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2026 = t2025 * t208;
    let t2027 = t2026 * t213;
    let t2029 = t871 * t97;
    let t2030 = t2029 * t588;
    let t2032 = t872 * t591;
    let t2034 = 2.0_f64 / 135.0_f64 * t1424;
    let t2035 = 2.0_f64 / 135.0_f64 * t1448;
    let t2036 = t1505 / 45.0_f64;
    let t2037 = t1518 / 45.0_f64;
    (t2026, t2027, t2029, t2030, t2032, t2034, t2035, t2036, t2037)
}
