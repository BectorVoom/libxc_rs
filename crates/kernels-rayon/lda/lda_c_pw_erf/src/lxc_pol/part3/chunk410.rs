//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 410/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk410(t1529: f64, t211: f64, t1471: f64, t1474: f64, t1478: f64, t1482: f64, t1490: f64, t1500: f64, t1510: f64, t1515: f64, t1517: f64, t1521: f64, t1526: f64, t1528: f64) -> (f64, f64) {
    let t1531 = 4.0_f64 / 135.0_f64 * t211 * t1529;
    let t1532 = -t1471 + t1474 + t1478 + t1482 + t1490 + t1500 + t1510 + t1515 + t1517 - t1521 + t1526 + t1528 - t1531;
    (t1531, t1532)
}
