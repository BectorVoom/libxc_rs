//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 707/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk707(t3439: f64, t6267: f64, t6269: f64, t6274: f64, t6279: f64, t6284: f64, t6289: f64, t6291: f64, t6294: f64, t6296: f64, t6299: f64, t6300: f64, t6302: f64, t6305: f64, t6308: f64, t6309: f64, t6310: f64) -> f64 {
    let t6311 = t6267 - t6269 + t6274 + t6279 - t6284 - t6289 + t6291 + t6294 - t6296 - t6299 - t6300 + t3439 + t6302 + t6305 + t6308 - t6309 + t6310;
    t6311
}
