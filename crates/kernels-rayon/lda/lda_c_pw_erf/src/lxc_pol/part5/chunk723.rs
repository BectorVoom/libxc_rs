//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 723/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk723(t2385: f64, t4753: f64, t3416: f64, t1954: f64, t743: f64, t4758: f64, t1318: f64, t2000: f64, t34: f64, t2023: f64, t2146: f64, t2433: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6475 = 16.0_f64 / 45.0_f64 * t4753 * t2385;
    let t6477 = 16.0_f64 / 45.0_f64 * t3416 * t2385;
    let t6478 = t1954 * t743;
    let t6479 = t4758 * t6478;
    let t6481 = 16.0_f64 / 45.0_f64 * t1318 * t6479;
    let t6482 = t2000 * t34;
    let t6483 = t4758 * t6482;
    let t6485 = 32.0_f64 / 45.0_f64 * t1318 * t6483;
    let t6487 = 8.0_f64 / 45.0_f64 * t2146 * t2023;
    let t6488 = t2433 * t542;
    (t6475, t6477, t6478, t6479, t6481, t6482, t6483, t6485, t6487, t6488)
}
