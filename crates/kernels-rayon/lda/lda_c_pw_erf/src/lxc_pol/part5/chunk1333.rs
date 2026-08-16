//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1333/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1333(t13420: f64, t17814: f64, t17816: f64, t17820: f64, t21771: f64, t21775: f64, t21776: f64, t21871: f64, t21875: f64, t21878: f64, t21881: f64, t21885: f64, t21888: f64) -> f64 {
    let t23275 = t21771 + t21775 - t21776 + t21871 - t13420 + 0.6492624817418906_f64 * t17814 + 0.21642082724729686_f64 * t17816 + 0.3246312408709453_f64 * t17820 - t21875 + t21878 - t21881 - t21885 + t21888;
    t23275
}
