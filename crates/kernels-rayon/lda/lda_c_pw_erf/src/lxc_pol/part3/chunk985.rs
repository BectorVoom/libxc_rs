//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 985/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk985(t11432: f64, t11458: f64, t59: f64, t40: f64, t87: f64, t1765: f64, t2948: f64, t1077: f64, t4393: f64, t11388: f64, t11390: f64, t11392: f64, t11398: f64, t11399: f64, t11402: f64, t11404: f64, t11406: f64, t8386: f64, t8389: f64, t8393: f64, t8397: f64, t8400: f64, t8403: f64, t8405: f64) -> (f64, f64, f64, f64, f64) {
    let t11460 = (t11432 + t11458) * t59;
    let t11462 = t40 * t11460 * t87;
    let t11463 = t1765 * t2948;
    let t11464 = 103.89453539625518_f64 * t11463;
    let t11465 = t4393 * t1077;
    let t11466 = 3.5089340384731225_f64 * t11465;
    let t11467 = t8386 - t11388 + t11390 - t11392 - t8389 - t8393 + t8397 - t8400 - 0.41076328840066667_f64 * t8403 + 2.0538164420033334_f64 * t8405 - t11398 + 3.1636214830824234_f64 * t11399 + t11402 + t11404 - t11406 + t11462 + t11464 + t11466;
    (t11460, t11462, t11464, t11466, t11467)
}
