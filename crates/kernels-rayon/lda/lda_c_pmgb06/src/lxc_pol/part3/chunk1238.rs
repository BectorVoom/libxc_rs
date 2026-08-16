//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1238/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1238(t2771: f64, t4351: f64, t247: f64, t4344: f64, t927: f64, t101: f64, t5915: f64, t754: f64, t757: f64, t10960: f64, t10964: f64, t10967: f64, t10976: f64, t10980: f64, t10985: f64, t10991: f64, t10993: f64, t11582: f64, t11637: f64, t11664: f64, t11684: f64, t14564: f64, t14606: f64, t14625: f64, t14752: f64, t2: f64, t328: f64, t8024: f64, t8028: f64, t8032: f64, t8034: f64, t8039: f64, t8043: f64, t8047: f64) -> f64 {
    let t14758 = t4351 * t2771;
    let t14761 = t247 * t927 * t4344;
    let t14765 = t101 * t5915 * t754 * t757;
    let tv4rho41 = 1.4220018064581168_f64 * t10967 - 1.2654485932329695_f64 * t10980 + t10985 + t8024 + 0.41076328840066667_f64 * t10960 + 1.898172889849454_f64 * t10964 + t10991 + 0.41076328840066667_f64 * t8028 + 3.0_f64 * t10993 + t2 * (t11582 + t11637 + t11664 + t11684 + t14564 + t14606 + t14625 + t14752) * t328 - 0.22820182688925925_f64 * t14758 + t10976 + 0.4740006021527056_f64 * t14761 + t8047 - t8039 + 1.232289865202_f64 * t14765 - 3.796345779698908_f64 * t8032 - 0.6846054806677778_f64 * t8034 + t8043;
    tv4rho41
}
