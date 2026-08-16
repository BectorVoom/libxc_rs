//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 733/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk733(t496: f64, t6597: f64, t202: f64, t2423: f64, t184: f64) -> (f64, f64, f64) {
    let t6599 = 4.0_f64 / 15.0_f64 * t6597 * t496;
    let t6600 = t202 * t2423;
    let t6601 = t6600 * t184;
    (t6599, t6600, t6601)
}
