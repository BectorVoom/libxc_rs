//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1329/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1329(t12999: f64, t13049: f64, t13052: f64, t13359: f64, t21680: f64, t21681: f64, t21683: f64, t21685: f64, t21687: f64, t21692: f64, t21694: f64, t21695: f64, t21696: f64) -> f64 {
    let t23258 = t12999 - t21680 - t21681 - t21683 + t21685 + t21687 + t21692 + t13049 + t13052 + t21694 + t21695 - t21696 - t13359;
    t23258
}
