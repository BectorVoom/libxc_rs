//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 671/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk671(t1210: f64, t168: f64, t861: f64, t153: f64, t1891: f64, t474: f64, t1729: f64, t452: f64, t454: f64, t1872: f64, t2765: f64, t1184: f64, t780: f64) -> (f64, f64, f64, f64, f64) {
    let t5907 = t168 * t1210 * t861;
    let t5911 = 1.1389037339096726_f64 * t153 * t474 * t1891;
    let t5924 = t1729 * t452 * t454;
    let t5925 = t2765 * t1872;
    let t5931 = t1184 * t780;
    (t5907, t5911, t5924, t5925, t5931)
}
