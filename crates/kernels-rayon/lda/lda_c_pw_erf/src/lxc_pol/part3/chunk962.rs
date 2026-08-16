//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 962/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk962(t1155: f64, t646: f64, t4100: f64, t10682: f64, t3921: f64, t256: f64, t3939: f64, t652: f64, t19: f64, t2853: f64, t644: f64, t647: f64) -> (f64, f64, f64, f64, f64) {
    let t11029 = 0.19208479012345678_f64 * t1155 * t646;
    let t11035 = t4100 * t646;
    let t11038 = 0.008082336938271605_f64 * t10682 * t3921;
    let t11046 = t3939 * t652 * t256;
    let t11050 = t2853 * t19 * t644 * t647;
    (t11029, t11035, t11038, t11046, t11050)
}
