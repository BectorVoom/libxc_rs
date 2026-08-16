//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 873/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk873(t286: f64, t3951: f64, t637: f64, t8131: f64, t3734: f64, t974: f64, t1022: f64, t1039: f64, t232: f64, t3669: f64, t8595: f64, t696: f64, t8522: f64, t963: f64, t967: f64) -> (f64, f64, f64, f64, f64) {
    let t8781 = t637 * t3951 * t286;
    let t8785 = t8131 * t286;
    let t8787 = t974 * t3734;
    let t8794 = 6207.121550312808_f64 * t232 / t1039 / t1022 * t8595 * t3669;
    let t8798 = 51.94757731704439_f64 * t696 * t963 * t8522 * t967;
    (t8781, t8785, t8787, t8794, t8798)
}
