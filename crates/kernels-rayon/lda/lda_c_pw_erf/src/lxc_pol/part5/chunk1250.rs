//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1250/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1250(t544: f64, t7466: f64, t2072: f64, t6601: f64, t511: f64, t7522: f64, t172: f64, t184: f64, t7659: f64, t496: f64, t14044: f64, t256: f64, t652: f64, t8032: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22403 = 4.0_f64 / 5.0_f64 * t7466 * t544;
    let t22405 = 4.0_f64 / 5.0_f64 * t6601 * t2072;
    let t22407 = 4.0_f64 / 5.0_f64 * t511 * t7522;
    let t22409 = t172 * t7659 * t184;
    let t22411 = 4.0_f64 / 15.0_f64 * t22409 * t496;
    let t22412 = 16.0_f64 / 135.0_f64 * t14044;
    let t22418 = t8032 * t652 * t256;
    (t22403, t22405, t22407, t22411, t22412, t22418)
}
