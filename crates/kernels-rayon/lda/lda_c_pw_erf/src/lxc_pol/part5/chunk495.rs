//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 495/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk495(t2419: f64, t557: f64, t11: f64, t1491: f64, t1941: f64, t2413: f64, t2417: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t2420 = t557 * t2419;
    let t2421 = t11 * t2420;
    let t2423 = -t1491 - 0.0012594444444444445_f64 * t1941 + 0.0012594444444444445_f64 * t2413 - 0.003778333333333333_f64 * t2417 + 0.0018891666666666666_f64 * t2421;
    let t2424 = t203 * t2423;
    let t2425 = t2424 * t184;
    (t2420, t2421, t2423, t2424, t2425)
}
