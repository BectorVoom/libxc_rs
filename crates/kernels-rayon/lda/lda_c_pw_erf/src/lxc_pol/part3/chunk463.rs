//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 463/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk463(t34: f64, t408: f64, t1820: f64, t1823: f64, t1826: f64, t348: f64, t352: f64, t462: f64) -> (f64, f64) {
    let t1829 = t408 * t34;
    let t1832 = -t1820 * t348 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t1823 * t462 - t1826 * t352 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t1829 * t462;
    (t1829, t1832)
}
