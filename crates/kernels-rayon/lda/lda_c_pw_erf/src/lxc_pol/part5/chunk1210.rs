//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1210/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1210(t11153: f64, t11156: f64, t11159: f64, t11166: f64, t11168: f64, t13420: f64, t21771: f64, t21775: f64, t21776: f64, t21871: f64, t21875: f64, t21878: f64, t21881: f64) -> f64 {
    let t21882 = t21771 + t21775 - t21776 + t21871 - t13420 - t11153 - t11156 + t11159 - 2.0_f64 / 9.0_f64 * t11166 - 0.013506172839506173_f64 * t11168 - t21875 + t21878 - t21881;
    t21882
}
