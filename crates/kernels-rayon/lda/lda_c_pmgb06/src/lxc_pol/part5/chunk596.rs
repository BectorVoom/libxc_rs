//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 596/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk596(t208: f64, t395: f64, t206: f64, t1730: f64, t573: f64, t580: f64, t122: f64, t1669: f64, t610: f64, t107: f64, t2786: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4159 = t395 * t208;
    let t4161 = 0.06649088888888889_f64 * t206 * t4159;
    let t4162 = t573 * t1730;
    let t4165 = 0.09973633333333333_f64 * t580 * t1730;
    let t4174 = t122 * t1669 * t610;
    let t4181 = 4.429070076315393_f64 * t107 * t2786 * t290;
    (t4159, t4161, t4162, t4165, t4174, t4181)
}
