//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1328/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1328(t10172: f64, t12975: f64, t21614: f64, t21617: f64, t21622: f64, t21624: f64, t21664: f64, t21665: f64, t21668: f64, t21675: f64, t21676: f64, t21677: f64, t21678: f64) -> f64 {
    let t23257 = -t21614 + t21617 - t21622 + t21624 - t10172 + t21664 - t21665 - t21668 - t12975 - t21675 + t21676 + t21677 - t21678;
    t23257
}
