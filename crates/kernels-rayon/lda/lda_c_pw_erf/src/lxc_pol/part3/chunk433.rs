//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 433/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk433(t1638: f64, t1639: f64, t1399: f64, t1407: f64, t1445: f64, t1448: f64, t1452: f64, t1456: f64, t1464: f64, t1471: f64, t1629: f64, t1632: f64, t1637: f64) -> (f64, f64) {
    let t1641 = 0.011181742741110338_f64 * t1638 * t1639;
    let t1642 = t1399 + t1407 + t1629 + 0.21642082724729686_f64 * t1632 + t1637 + t1641 - t1445 + t1448 + t1452 + t1456 + t1464 - t1471;
    (t1641, t1642)
}
