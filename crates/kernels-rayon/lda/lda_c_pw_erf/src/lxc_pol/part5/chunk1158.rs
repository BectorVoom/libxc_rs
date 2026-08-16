//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1158/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1158(t34: f64, t6351: f64, t519: f64, t5256: f64, t1472: f64, t7728: f64, t13060: f64, t571: f64, t6270: f64, t4763: f64, t6405: f64, t1318: f64, t2017: f64, t549: f64, t7414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21246 = t6351 * t34;
    let t21249 = 16.0_f64 / 3.0_f64 * t519 * t5256 * t21246;
    let t21251 = 8.0_f64 / 15.0_f64 * t1472 * t7728;
    let t21255 = 16.0_f64 / 15.0_f64 * t571 * t13060 * t6270 * t34;
    let t21257 = 16.0_f64 / 15.0_f64 * t4763 * t6405;
    let t21261 = 16.0_f64 / 9.0_f64 * t1318 * t2017 * t7414 * t549;
    (t21246, t21249, t21251, t21255, t21257, t21261)
}
