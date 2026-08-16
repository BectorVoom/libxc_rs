//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1093/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1093(t3104: f64, t5083: f64, t823: f64, t2961: f64, t5078: f64, t12535: f64, t1435: f64, t5075: f64, t4744: f64, t477: f64, t5084: f64, t3259: f64, t5066: f64) -> (f64, f64, f64, f64, f64) {
    let t13015 = t5083 * t823 * t3104 / 9.0_f64;
    let t13018 = 2.0_f64 / 9.0_f64 * t5083 * t5078 * t2961;
    let t13020 = t5075 * t12535 * t1435;
    let t13021 = t4744 * t477;
    let t13024 = 4.0_f64 / 9.0_f64 * t13020 * t5084 * t13021;
    let t13026 = t5075 * t5066 * t3259;
    (t13015, t13018, t13021, t13024, t13026)
}
