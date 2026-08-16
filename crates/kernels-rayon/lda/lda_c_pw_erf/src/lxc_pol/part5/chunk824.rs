//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 824/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk824(t1466: f64, t7557: f64, t571: f64, t7007: f64, t826: f64, t6205: f64, t6198: f64, t799: f64, t2532: f64, t4763: f64, t6188: f64, t811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7558 = t1466 * t7557;
    let t7560 = 4.0_f64 / 5.0_f64 * t571 * t7558;
    let t7562 = 8.0_f64 / 15.0_f64 * t7007 * t826;
    let t7564 = 4.0_f64 / 15.0_f64 * t6205 * t826;
    let t7566 = 4.0_f64 / 15.0_f64 * t6198 * t799;
    let t7568 = 8.0_f64 / 5.0_f64 * t4763 * t2532;
    let t7569 = t6188 * t811;
    (t7558, t7560, t7562, t7564, t7566, t7568, t7569)
}
