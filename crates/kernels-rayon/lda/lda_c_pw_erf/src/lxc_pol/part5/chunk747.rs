//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 747/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk747(t5176: f64, t4029: f64, t2405: f64, t509: f64, t184: f64, t199: f64, t2407: f64, t515: f64, t2523: f64, t331: f64, t2517: f64, t2520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6785 = 16.0_f64 / 45.0_f64 * t5176;
    let t6786 = 4.0_f64 / 135.0_f64 * t4029;
    let t6787 = t2405 * t509;
    let t6788 = t6787 * t184;
    let t6790 = 4.0_f64 / 15.0_f64 * t6788 * t199;
    let t6791 = t2407 * t515;
    let t6792 = 8.0_f64 / 45.0_f64 * t6791;
    let t6793 = t331 * t2523;
    let t6795 = t331 * t2517;
    let t6797 = t331 * t2520;
    (t6785, t6786, t6787, t6788, t6790, t6791, t6792, t6793, t6795, t6797)
}
