//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 974/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk974(t11317: f64, t2701: f64, t4397: f64, t1085: f64, t1798: f64, t4: f64, t8189: f64, t1769: f64, t4295: f64, t2851: f64, t749: f64, t1: f64, t397: f64, t4383: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11318 = 0.016265371324172287_f64 * t11317;
    let t11319 = t4397 * t2701;
    let t11320 = 0.4815944609513912_f64 * t11319;
    let t11322 = t1798 * t4 * t1085;
    let t11323 = 0.032530742648344574_f64 * t11322;
    let t11324 = 1.7544670192365612_f64 * t8189;
    let t11325 = t1769 * t4295;
    let t11327 = t2851 * t749;
    let t11328 = 144.0_f64 * t11327;
    let t11330 = t4383 * t1 * t397;
    (t11318, t11320, t11323, t11324, t11325, t11328, t11330)
}
