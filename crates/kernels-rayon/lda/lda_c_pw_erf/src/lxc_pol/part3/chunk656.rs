//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 656/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk656(t1415: f64, t652: f64, t256: f64, t1112: f64, t19: f64, t644: f64, t647: f64, t1432: f64, t639: f64, t1423: f64, t1427: f64, t1991: f64, t3482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3943 = t1415 * t652;
    let t3944 = t3943 * t256;
    let t3945 = t1112 * t19;
    let t3946 = t3945 * t644;
    let t3947 = t3946 * t647;
    let t3949 = t639 * t1432;
    let t3950 = t3949 * t256;
    let t3951 = t1423 * t1427;
    let t3953 = t1991 * t3482;
    (t3943, t3944, t3945, t3946, t3947, t3949, t3950, t3951, t3953)
}
