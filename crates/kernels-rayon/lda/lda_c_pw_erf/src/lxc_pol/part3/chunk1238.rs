//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1238/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1238(t14646: f64, t5607: f64, t4: f64, t411: f64, t474: f64, t2: f64, t39: f64, t756: f64, t8901: f64, t102: f64, t436: f64, t1568: f64, t1872: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14647 = t5607 * t14646;
    let t14648 = 2.923025_f64 * t14647;
    let t14650 = t4 * t474 * t411;
    let t14651 = t5607 * t14650;
    let t14652 = 3.8973666666666666_f64 * t14651;
    let t14654 = t756 * t2 * t39;
    let t14655 = t8901 * t14654;
    let t14656 = 1.9486833333333333_f64 * t14655;
    let t14657 = t102 * t436;
    let t14658 = t1872 * t1568;
    (t14648, t14650, t14652, t14654, t14656, t14657, t14658)
}
