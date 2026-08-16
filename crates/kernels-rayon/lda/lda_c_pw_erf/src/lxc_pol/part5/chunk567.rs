//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 567/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk567(t1064: f64, t358: f64, t1039: f64, t339: f64, t344: f64, t1191: f64, t169: f64, t301: f64, t678: f64, t119: f64, t411: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3179 = t1064 * t358;
    let t3180 = 60.0_f64 * t3179;
    let t3181 = t339 * t1039;
    let t3182 = 24.0_f64 * t3181;
    let t3183 = t344 * t1039;
    let t3184 = 24.0_f64 * t3183;
    let t3203 = t169 * t1191 * t678 * t301;
    let t3216 = t119 * t473 * t411;
    (t3179, t3180, t3181, t3182, t3183, t3184, t3203, t3216)
}
