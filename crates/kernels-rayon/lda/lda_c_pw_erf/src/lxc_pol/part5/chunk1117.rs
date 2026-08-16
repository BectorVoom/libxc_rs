//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1117/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1117(t1967: f64, t2497: f64, t4488: f64, t4500: f64, t2131: f64, t6597: f64, t11900: f64, t2329: f64, t806: f64, t348: f64, t4494: f64, t4501: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20773 = 4.0_f64 / 9.0_f64 * t4488 * t4500 * t2497 * t1967;
    let t20775 = 4.0_f64 / 5.0_f64 * t6597 * t2131;
    let t20776 = 8.0_f64 / 45.0_f64 * t11900;
    let t20777 = t2329 * t806;
    let t20778 = t20777 * t348;
    let t20781 = 8.0_f64 / 15.0_f64 * t4488 * t4494 * t20778;
    let t20784 = 4.0_f64 / 9.0_f64 * t4488 * t4501 * t20778;
    (t20773, t20775, t20776, t20777, t20781, t20784)
}
