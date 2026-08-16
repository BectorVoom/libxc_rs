//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 888/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk888(t3238: f64, t517: f64, t1710: f64, t431: f64, t2010: f64, t446: f64, t1981: f64, t500: f64, t1680: f64, t1688: f64, t1691: f64, t4119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10293 = t3238 * t517;
    let t10318 = t431 * t1710;
    let t10321 = t2010 * t446;
    let t10335 = t1981 * t500;
    let t10350 = t1688 * t1680;
    let t10353 = t1691 * t4119;
    (t10293, t10318, t10321, t10335, t10350, t10353)
}
