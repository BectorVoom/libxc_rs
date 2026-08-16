//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 201/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk201(t220: f64, t549: f64, t186: f64, t548: f64, t174: f64, t205: f64, t499: f64, t213: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t550 = t220 * t549;
    let t551 = t186 * t550;
    let t553 = 4.0_f64 / 15.0_f64 * t548 * t551;
    let t555 = t174 * t499 * t205;
    let t556 = 0.0018891666666666666_f64 * t555;
    let t557 = t56 * t213;
    (t550, t551, t553, t555, t556, t557)
}
