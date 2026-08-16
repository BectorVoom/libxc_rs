//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 236/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk236(t286: f64, t684: f64, t159: f64, t285: f64, t465: f64, t147: f64, t477: f64, t281: f64, t462: f64) -> (f64, f64, f64, f64, f64) {
    let t686 = 0.019957056683757683_f64 * t684 * t286;
    let t688 = t465 * t159 * t285;
    let t692 = t147 * t477 * t285;
    let t694 = 0.01197423401025461_f64 * t281 * t692;
    let t695 = t462 * t147;
    (t686, t688, t692, t694, t695)
}
