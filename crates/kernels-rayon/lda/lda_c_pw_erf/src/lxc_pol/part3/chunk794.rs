//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 794/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk794(t525: f64, t5327: f64, t2158: f64, t3416: f64, t1472: f64, t2163: f64, t1959: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t5329 = 8.0_f64 / 45.0_f64 * t5327 * t525;
    let t5331 = 8.0_f64 / 15.0_f64 * t3416 * t2158;
    let t5333 = 8.0_f64 / 15.0_f64 * t1472 * t2163;
    let t5334 = t1959 * t518;
    (t5329, t5331, t5333, t5334)
}
