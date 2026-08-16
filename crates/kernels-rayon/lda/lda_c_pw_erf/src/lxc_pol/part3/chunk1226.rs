//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1226/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1226(t8482: f64, t8486: f64, t8493: f64, t8495: f64, t8497: f64, t8499: f64, t1064: f64, t1775: f64, t8510: f64, t8469: f64, t8473: f64, t8477: f64, t8481: f64, t8491: f64, t8505: f64, t8509: f64, t8516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14429 = 10.526802115419367_f64 * t8482;
    let t14430 = 155.84180309438278_f64 * t8486;
    let t14431 = 3.0_f64 * t8493;
    let t14432 = 36.0_f64 * t8495;
    let t14433 = 96.0_f64 * t8497;
    let t14434 = 36.0_f64 * t8499;
    let t14435 = t1064 * t1775;
    let t14436 = 60.0_f64 * t14435;
    let t14437 = 96.0_f64 * t8510;
    let t14438 = t8469 + t8473 - t8477 - t8481 + t14429 - t14430 + t8491 + t14431 - t14432 - t14433 - t14434 - t8505 + t8509 + t14436 - t14437 + t8516;
    (t14429, t14430, t14431, t14432, t14433, t14434, t14436, t14437, t14438)
}
