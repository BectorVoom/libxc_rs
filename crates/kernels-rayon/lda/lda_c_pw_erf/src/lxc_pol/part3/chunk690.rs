//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 690/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk690(t4176: f64, t4177: f64, t4179: f64, t4180: f64, t4210: f64, t4211: f64, t4213: f64, t4236: f64, t163: f64, t1645: f64, t169: f64, t299: f64) -> (f64, f64) {
    let t4239 = t4176 + t4177 + t4179 + t4180 + t4210 + t4211 + t4213 + t4236;
    let t4246 = t169 * t299 * t1645 * t163;
    (t4239, t4246)
}
