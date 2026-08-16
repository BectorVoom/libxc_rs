//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 899/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk899(t153: f64, t343: f64, t4606: f64, t5021: f64, t8798: f64, t147: f64, t281: f64, t285: f64, t1138: f64, t2817: f64, t2820: f64, t465: f64) -> (f64, f64, f64) {
    let t8801 = 0.017888888888888888_f64 * t4606 + 0.22252592592592593_f64 * t5021 - 0.07316671043820612_f64 * t343 + 0.015663796296296297_f64 * t153 * t8798;
    let t8805 = 0.01197423401025461_f64 * t281 * t147 * t8801 * t285;
    let t8808 = t2817 * t465 * t1138 * t2820;
    (t8801, t8805, t8808)
}
