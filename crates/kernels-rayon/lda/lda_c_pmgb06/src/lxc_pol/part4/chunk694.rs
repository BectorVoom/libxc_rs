//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 694/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk694(t1980: f64, t223: f64, t208: f64, t395: f64, t206: f64, t1730: f64, t573: f64, t580: f64, t122: f64, t1669: f64, t610: f64, t1735: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4151 = 8.0_f64 / 405.0_f64 * t223 * t1980;
    let t4159 = t395 * t208;
    let t4161 = 0.06649088888888889_f64 * t206 * t4159;
    let t4162 = t573 * t1730;
    let t4165 = 0.09973633333333333_f64 * t580 * t1730;
    let t4174 = t122 * t1669 * t610;
    let t4177 = t122 * t569 * t1735;
    (t4151, t4159, t4161, t4162, t4165, t4174, t4177)
}
