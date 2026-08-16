//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 430/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk430(t1308: f64, t2010: f64, t571: f64, t1319: f64, t1949: f64, t1485: f64, t219: f64) -> (f64, f64, f64, f64, f64) {
    let t2011 = t1308 * t2010;
    let t2013 = 4.0_f64 / 45.0_f64 * t571 * t2011;
    let t2014 = t1319 * t1949;
    let t2016 = 8.0_f64 / 45.0_f64 * t571 * t2014;
    let t2017 = t1485 * t219;
    (t2011, t2013, t2014, t2016, t2017)
}
