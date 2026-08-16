//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 918/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk918(t11608: f64, t1186: f64, t1770: f64, t5899: f64, t1193: f64, t4001: f64, t4299: f64, t794: f64, t4320: f64, t909: f64, t123: f64, t317: f64, t902: f64) -> (f64, f64, f64, f64, f64) {
    let t11609 = 5.4655730795145296e-05_f64 * t11608;
    let t11611 = t5899 * t1186 * t1770;
    let t11615 = t4001 * t794 * t1193 * t4299;
    let t11617 = t4320 * t909;
    let t11624 = t123 * t4001 * t902 * t317;
    (t11609, t11611, t11615, t11617, t11624)
}
