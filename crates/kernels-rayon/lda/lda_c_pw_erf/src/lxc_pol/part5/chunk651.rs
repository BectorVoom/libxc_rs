//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 651/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk651(t2022: f64, t3863: f64, t571: f64, t1333: f64, t833: f64, t2026: f64, t3859: f64, t1325: f64, t1981: f64, t518: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5302 = t3863 * t2022;
    let t5304 = 16.0_f64 / 135.0_f64 * t571 * t5302;
    let t5305 = t833 * t1333;
    let t5310 = t3859 * t2026;
    let t5312 = 32.0_f64 / 135.0_f64 * t1325 * t5310;
    let t5327 = t1981 * t518;
    (t5302, t5304, t5305, t5310, t5312, t5327)
}
