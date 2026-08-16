//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 928/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk928(t325: f64, t3624: f64, t1953: f64, t560: f64, t1357: f64, t925: f64, t3643: f64, t3651: f64, t1353: f64, t3634: f64, t4048: f64, t56: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10079 = t325 * t3624;
    let t10090 = t1953 * t560;
    let t10092 = t925 * t1357;
    let t10094 = t325 * t3643;
    let t10096 = t325 * t3651;
    let t10098 = t925 * t1353;
    let t10100 = t325 * t3634;
    let t10102 = t56 * t4048;
    (t10079, t10090, t10092, t10094, t10096, t10098, t10100, t10102)
}
