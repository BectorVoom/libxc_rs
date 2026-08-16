//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1144/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1144(t504: f64, t7797: f64, t1325: f64, t1326: f64, t348: f64, t3794: f64, t7738: f64, t1446: f64, t7742: f64, t15926: f64, t6277: f64, t4738: f64, t6265: f64) -> (f64, f64, f64, f64, f64) {
    let t21079 = t7797 * t504;
    let t21083 = 8.0_f64 / 45.0_f64 * t1325 * t1326 * t21079 * t348;
    let t21085 = 16.0_f64 / 15.0_f64 * t3794 * t7738;
    let t21087 = 8.0_f64 / 15.0_f64 * t1446 * t7742;
    let t21089 = 8.0_f64 / 15.0_f64 * t15926 * t6277;
    let t21091 = 8.0_f64 / 15.0_f64 * t4738 * t6265;
    (t21083, t21085, t21087, t21089, t21091)
}
