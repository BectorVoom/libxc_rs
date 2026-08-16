//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1182/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1182(t15926: f64, t6993: f64, t581: f64, t7456: f64, t1318: f64, t1466: f64, t549: f64, t15619: f64, t571: f64, t833: f64, t1440: f64, t2098: f64, t519: f64, t7002: f64) -> (f64, f64, f64, f64) {
    let t21500 = 4.0_f64 / 5.0_f64 * t15926 * t6993;
    let t21501 = t581 * t7456;
    let t21505 = 4.0_f64 / 15.0_f64 * t1318 * t1466 * t21501 * t549;
    let t21509 = 4.0_f64 / 5.0_f64 * t571 * t1466 * t15619 * t833;
    let t21513 = 4.0_f64 / 5.0_f64 * t519 * t1440 * t7002 * t2098;
    (t21500, t21505, t21509, t21513)
}
