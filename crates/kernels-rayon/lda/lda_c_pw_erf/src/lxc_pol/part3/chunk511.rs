//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 511/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk511(t2137: f64, t548: f64, t1475: f64, t825: f64, t571: f64, t1449: f64, t798: f64, t519: f64, t518: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2138 = t548 * t2137;
    let t2139 = 8.0_f64 / 45.0_f64 * t2138;
    let t2140 = t1475 * t825;
    let t2141 = t571 * t2140;
    let t2142 = 8.0_f64 / 135.0_f64 * t2141;
    let t2143 = t1449 * t798;
    let t2144 = t519 * t2143;
    let t2145 = 8.0_f64 / 135.0_f64 * t2144;
    let t2146 = t821 * t518;
    (t2139, t2140, t2142, t2143, t2145, t2146)
}
