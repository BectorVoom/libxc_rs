//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 914/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk914(t174: f64, t205: f64, t9810: f64, t1332: f64, t1350: f64, t1953: f64, t560: f64, t4048: f64, t56: f64, t9812: f64, t155: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10042 = t174 * t9810 * t205;
    let t10043 = 0.01959135802469136_f64 * t10042;
    let t10056 = 1.0_f64 / t1350 / t1332;
    let t10090 = t1953 * t560;
    let t10102 = t56 * t4048;
    let t10145 = 0.01959135802469136_f64 * t9812;
    let t10162 = t155 * t188;
    (t10042, t10043, t10056, t10090, t10102, t10145, t10162)
}
