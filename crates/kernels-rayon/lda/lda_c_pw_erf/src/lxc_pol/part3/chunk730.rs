//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 730/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk730(t1243: f64, t4620: f64, t1953: f64, t1966: f64, t945: f64, t11: f64, t940: f64, t503: f64, t1251: f64, t34: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4621 = t1243 * t4620;
    let t4622 = t1953 * t4621;
    let t4624 = t1966 * t945;
    let t4625 = t1243 * t4624;
    let t4626 = t11 * t4625;
    let t4628 = t1966 * t940;
    let t4629 = t503 * t4628;
    let t4630 = t11 * t4629;
    let t4632 = t1251 * t34;
    let t4633 = t4632 * t348;
    let t4634 = t503 * t4633;
    let t4635 = t1953 * t4634;
    (t4621, t4622, t4624, t4625, t4626, t4628, t4629, t4630, t4632, t4633, t4634, t4635)
}
