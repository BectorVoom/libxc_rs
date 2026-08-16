//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 646/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk646(t3841: f64, t571: f64, t1315: f64, t1446: f64, t1256: f64, t542: f64, t1313: f64, t519: f64, t1278: f64, t505: f64, t1475: f64, t219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3843 = 8.0_f64 / 15.0_f64 * t571 * t3841;
    let t3845 = 8.0_f64 / 15.0_f64 * t1446 * t1315;
    let t3846 = t1256 * t542;
    let t3847 = t1313 * t3846;
    let t3849 = 4.0_f64 / 15.0_f64 * t519 * t3847;
    let t3850 = t505 * t1278;
    let t3851 = t1313 * t3850;
    let t3853 = 4.0_f64 / 15.0_f64 * t519 * t3851;
    let t3854 = t1475 * t219;
    (t3843, t3845, t3846, t3847, t3849, t3850, t3851, t3853, t3854)
}
