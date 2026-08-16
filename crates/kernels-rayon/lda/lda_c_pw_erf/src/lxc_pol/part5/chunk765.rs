//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 765/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk765(t41: f64, t6039: f64, t2379: f64, t632: f64, t153: f64, t2357: f64, t474: f64, t168: f64, t2581: f64, t635: f64, t145: f64, t2363: f64) -> (f64, f64, f64, f64, f64) {
    let t7032 = t41 * t6039;
    let t7035 = t2379 * t632;
    let t7038 = t153 * t474 * t2357;
    let t7043 = t168 * t635 * t2581;
    let t7045 = t145 * t2363;
    (t7032, t7035, t7038, t7043, t7045)
}
