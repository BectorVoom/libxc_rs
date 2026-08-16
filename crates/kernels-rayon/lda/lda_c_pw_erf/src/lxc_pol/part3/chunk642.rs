//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 642/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk642(t1443: f64, t3794: f64, t529: f64, t542: f64, t944: f64, t1440: f64, t1325: f64, t1449: f64) -> (f64, f64, f64, f64, f64) {
    let t3796 = 8.0_f64 / 5.0_f64 * t3794 * t1443;
    let t3798 = t529 * t944 * t542;
    let t3799 = t1440 * t3798;
    let t3801 = 4.0_f64 / 5.0_f64 * t1325 * t3799;
    let t3802 = t1449 * t529;
    (t3796, t3798, t3799, t3801, t3802)
}
