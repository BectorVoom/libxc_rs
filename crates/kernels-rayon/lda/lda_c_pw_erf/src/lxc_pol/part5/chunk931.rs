//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 931/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk931(t1426: f64, t695: f64, t3926: f64, t458: f64, t1155: f64, t646: f64, t10682: f64, t3921: f64, t3949: f64, t656: f64, t1423: f64, t3915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11025 = 0.26596355555555556_f64 * t695 * t1426;
    let t11027 = 0.19947266666666666_f64 * t458 * t3926;
    let t11029 = 0.19208479012345678_f64 * t1155 * t646;
    let t11038 = 0.008082336938271605_f64 * t10682 * t3921;
    let t11057 = t3949 * t656;
    let t11060 = t1423 * t3915;
    (t11025, t11027, t11029, t11038, t11057, t11060)
}
