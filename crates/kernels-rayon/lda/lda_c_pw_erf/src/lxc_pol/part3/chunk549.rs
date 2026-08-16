//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 549/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk549(t159: f64, t285: f64, t2853: f64, t1112: f64, t477: f64, t281: f64, t1128: f64, t465: f64, t1184: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2855 = t2853 * t159 * t285;
    let t2859 = t1112 * t477 * t285;
    let t2860 = t281 * t2859;
    let t2863 = t465 * t1128 * t285;
    let t2864 = t281 * t2863;
    let t2869 = t6 * t1184;
    (t2855, t2859, t2860, t2863, t2864, t2869)
}
