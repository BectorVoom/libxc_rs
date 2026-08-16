//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1402/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1402(t1680: f64, t2527: f64, t12304: f64, t12307: f64, t12308: f64, t12310: f64, t12312: f64, t16297: f64, t16299: f64, t16300: f64, t16301: f64, t16302: f64, t16303: f64, t16306: f64, t16308: f64, t16310: f64) -> f64 {
    let t18225 = t2527 * t1680;
    let t18227 = -t16297 - t16299 + t16300 + t16301 + t16302 + t16303 - t16306 + 4.0_f64 / 3.0_f64 * t12304 + t12307 + 4.0_f64 / 9.0_f64 * t12308 + 16.0_f64 / 9.0_f64 * t12310 + (4e-21_f64 as f64) * t12312 - t16308 + t16310 - 2.0_f64 / 27.0_f64 * t18225;
    t18227
}
