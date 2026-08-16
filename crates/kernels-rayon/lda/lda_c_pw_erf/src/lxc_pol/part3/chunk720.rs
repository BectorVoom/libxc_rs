//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 720/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk720(t593: f64, t743: f64, t352: f64, t4515: f64, t4506: f64, t1484: f64, t581: f64, t1351: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4516 = t743 * t593;
    let t4517 = t4516 * t352;
    let t4518 = t4515 * t4517;
    let t4520 = 16.0_f64 / 45.0_f64 * t4506 * t4518;
    let t4521 = t1484 * t581;
    let t4522 = t4521 * t1351;
    (t4516, t4517, t4518, t4520, t4521, t4522)
}
