//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 506/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk506(t2497: f64, t530: f64, t186: f64, t185: f64, t2120: f64, t786: f64, t198: f64, t2328: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2498 = t530 * t2497;
    let t2499 = t186 * t2498;
    let t2501 = 2.0_f64 / 15.0_f64 * t185 * t2499;
    let t2503 = 8.0_f64 / 15.0_f64 * t2120 * t786;
    let t2504 = t198 * t2328;
    let t2505 = t186 * t2504;
    (t2498, t2499, t2501, t2503, t2504, t2505)
}
