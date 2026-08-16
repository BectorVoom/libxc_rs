//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 755/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk755(t2531: f64, t3899: f64, t1318: f64, t2140: f64, t2146: f64, t2471: f64, t3675: f64, t542: f64, t1440: f64, t2098: f64, t2186: f64, t2166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6894 = t3899 * t2531;
    let t6895 = t1318 * t6894;
    let t6897 = t2146 * t2140;
    let t6903 = t3675 * t2471;
    let t6904 = t6903 * t542;
    let t6905 = t1440 * t6904;
    let t6908 = t2186 * t2098;
    let t6909 = t1440 * t6908;
    let t6916 = t2166 * t2098;
    (t6894, t6895, t6897, t6903, t6904, t6905, t6908, t6909, t6916)
}
