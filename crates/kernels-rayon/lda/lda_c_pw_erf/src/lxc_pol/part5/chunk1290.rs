//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1290/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1290(t4804: f64, t7601: f64, t3794: f64, t18673: f64, t18695: f64, t565: f64, t7458: f64, t211: f64, t514: f64, t7457: f64, t18710: f64, t18712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23067 = 4.0_f64 / 5.0_f64 * t4804 * t7601;
    let t23069 = 4.0_f64 / 5.0_f64 * t3794 * t7601;
    let t23070 = 16.0_f64 / 45.0_f64 * t18673;
    let t23071 = 8.0_f64 / 15.0_f64 * t18695;
    let t23073 = 2.0_f64 / 15.0_f64 * t565 * t7458;
    let t23075 = t211 * t514 * t7457;
    let t23076 = 4.0_f64 / 45.0_f64 * t23075;
    let t23077 = 4.0_f64 / 15.0_f64 * t18710;
    let t23078 = 8.0_f64 / 15.0_f64 * t18712;
    (t23067, t23069, t23070, t23071, t23073, t23076, t23077, t23078)
}
