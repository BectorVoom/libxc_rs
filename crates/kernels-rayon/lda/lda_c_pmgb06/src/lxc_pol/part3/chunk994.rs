//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 994/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk994(t2029: f64, t4119: f64, t9311: f64, t9313: f64, t1629: f64, t1966: f64, t439: f64, t5201: f64, t224: f64, t4753: f64, t446: f64, t1427: f64, t5187: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11813 = t2029 * t4119;
    let t11815 = 4.0_f64 / 45.0_f64 * t9311;
    let t11816 = 4.0_f64 / 45.0_f64 * t9313;
    let t11820 = t439 * t1966 * t5201 * t1629 / 5.0_f64;
    let t11821 = t4753 * t224;
    let t11823 = t11821 * t446 / 15.0_f64;
    let t11825 = 2.0_f64 / 15.0_f64 * t5187 * t1427;
    (t11813, t11815, t11816, t11820, t11823, t11825)
}
