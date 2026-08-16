//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1291/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1291(t211: f64, t514: f64, t7514: f64, t565: f64, t7515: f64, t14256: f64, t14314: f64, t14352: f64, t14366: f64, t23067: f64, t23069: f64, t23070: f64, t23071: f64, t23073: f64, t23076: f64, t23077: f64, t23078: f64) -> (f64, f64, f64) {
    let t23080 = t211 * t514 * t7514;
    let t23081 = 8.0_f64 / 15.0_f64 * t23080;
    let t23083 = 4.0_f64 / 5.0_f64 * t565 * t7515;
    let t23084 = -t14256 - t23067 - t23069 - t23070 + t23071 - t23073 + t14314 - t14352 - t23076 - t23077 + t23078 - t23081 - t23083 + t14366;
    (t23081, t23083, t23084)
}
