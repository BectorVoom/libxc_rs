//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 919/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk919(t156: f64, t4195: f64, t602: f64, t1: f64, t1185: f64, t119: f64, t603: f64, t1631: f64, t4204: f64, t4183: f64, t1634: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10675 = 0.4328416544945937_f64 * t602 * t156 * t4195;
    let t10682 = t1185 * t1;
    let t10685 = 2.8503734567901235e-05_f64 * t10682 * t119 * t603;
    let t10688 = t1631 * t4204;
    let t10690 = t1631 * t4183;
    let t10694 = 0.38474813732852775_f64 * t602 * t474 * t1634;
    (t10675, t10682, t10685, t10688, t10690, t10694)
}
