//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1189/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1189(t12765: f64, t21583: f64, t519: f64, t542: f64, t1325: f64, t2497: f64, t5289: f64, t784: f64, t1318: f64, t2478: f64, t5269: f64, t593: f64, t833: f64) -> (f64, f64, f64) {
    let t21591 = 12.0_f64 / 5.0_f64 * t519 * t12765 * t21583 * t542;
    let t21596 = 8.0_f64 / 5.0_f64 * t1325 * t5289 * t2497 * t784 * t542;
    let t21601 = 8.0_f64 / 5.0_f64 * t1318 * t5269 * t2478 * t833 * t593;
    (t21591, t21596, t21601)
}
