//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 976/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk976(t14313: f64, t1519: f64, t1982: f64, t1518: f64, t2066: f64, t211: f64, t1131: f64, t485: f64, t5474: f64, t1910: f64, t2910: f64, t1124: f64, t1904: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14314 = 8.0_f64 / 45.0_f64 * t14313;
    let t14351 = t1982 * t1519;
    let t14352 = 4.0_f64 / 45.0_f64 * t14351;
    let t14365 = t211 * t1518 * t2066;
    let t14366 = 4.0_f64 / 45.0_f64 * t14365;
    let t14385 = t5474 * t1131 * t485;
    let t14386 = 0.01185233419734569_f64 * t14385;
    let t14388 = t1910 * t2910 * t485;
    let t14392 = t1124 * t1904 * t483 * t485;
    (t14314, t14352, t14366, t14386, t14388, t14392)
}
