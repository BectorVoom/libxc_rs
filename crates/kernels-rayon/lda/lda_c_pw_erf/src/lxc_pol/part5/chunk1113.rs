//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1113/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1113(t2329: f64, t739: f64, t494: f64, t3965: f64, t5147: f64, t542: f64, t4488: f64, t4501: f64, t348: f64) -> (f64, f64, f64, f64, f64) {
    let t20728 = t2329 * t739;
    let t20729 = t20728 * t494;
    let t20732 = 8.0_f64 / 9.0_f64 * t3965 * t5147 * t20729;
    let t20733 = t20728 * t542;
    let t20736 = 4.0_f64 / 9.0_f64 * t4488 * t4501 * t20733;
    let t20737 = t20728 * t348;
    (t20729, t20732, t20733, t20736, t20737)
}
