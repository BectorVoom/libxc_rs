//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 368/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk368(t1319: f64, t1321: f64, t1318: f64, t492: f64, t518: f64) -> (f64, f64, f64) {
    let t1322 = t1319 * t1321;
    let t1324 = 16.0_f64 / 45.0_f64 * t1318 * t1322;
    let t1325 = t492 * t518;
    (t1322, t1324, t1325)
}
