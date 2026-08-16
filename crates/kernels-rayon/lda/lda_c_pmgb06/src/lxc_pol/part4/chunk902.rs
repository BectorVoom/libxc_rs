//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 902/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk902(t493: f64, t6533: f64, t1982: f64, t1988: f64, t1981: f64, t1444: f64, t2466: f64, t1450: f64, t2465: f64, t498: f64, t5974: f64, t496: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6535 = 2.0_f64 / 45.0_f64 * t493 * t6533;
    let t6536 = t1988 * t1982;
    let t6538 = 4.0_f64 / 45.0_f64 * t1981 * t6536;
    let t6540 = t1444 * t2466 / 45.0_f64;
    let t6541 = t1450 * t2465;
    let t6543 = t493 * t6541 / 45.0_f64;
    let t6544 = t498 * t5974;
    let t6545 = t496 * t6544;
    (t6535, t6536, t6538, t6540, t6541, t6543, t6544, t6545)
}
