//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 499/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk499(t1936: f64, t205: f64, t2414: f64, t208: f64, t1998: f64, t1679: f64, t1682: f64, t1700: f64, t1703: f64, t1939: f64, t213: f64, t224: f64, t2519: f64, t2522: f64, t2523: f64, t2524: f64) -> (f64, f64, f64, f64, f64) {
    let t2525 = 2.0_f64 / 45.0_f64 * t1936;
    let t2526 = t2414 * t205;
    let t2527 = t2526 * t208;
    let t2531 = 4.0_f64 / 135.0_f64 * t1998;
    let t2532 = t1679 - t1682 + t1700 + t1703 - t2519 * t224 / 15.0_f64 + t2522 + t2523 + t2524 + t2525 + t2527 * t213 / 3.0_f64 - 4.0_f64 / 45.0_f64 * t1939 + t2531;
    (t2525, t2526, t2527, t2531, t2532)
}
