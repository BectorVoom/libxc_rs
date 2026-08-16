//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 662/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk662(t12: f64, t1072: f64, t2200: f64, t336: f64, t337: f64, t5966: f64, t5971: f64, t5974: f64, t5965: f64, zeta_threshold: f64) -> f64 {
    let t13 = t12 <= zeta_threshold;
    let t5978 = piecewise3(t13, 0.0_f64, 8.0_f64 / 27.0_f64 * t5966 * t337 + 8.0_f64 / 9.0_f64 * t2200 * t1072 - 2.0_f64 / 9.0_f64 * t5971 * t337 + 2.0_f64 / 3.0_f64 * t336 * t5974);
    let t5980 = t5965 / 2.0_f64 + t5978 / 2.0_f64;
    t5980
}
