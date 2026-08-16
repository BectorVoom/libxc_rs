//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 694/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk694(t525: f64, t6198: f64, t2158: f64, t4763: f64, t2146: f64, t2163: f64, t2424: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t6200 = 4.0_f64 / 45.0_f64 * t6198 * t525;
    let t6202 = 8.0_f64 / 15.0_f64 * t4763 * t2158;
    let t6204 = 8.0_f64 / 15.0_f64 * t2146 * t2163;
    let t6205 = t2424 * t518;
    (t6200, t6202, t6204, t6205)
}
