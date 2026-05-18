//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1238/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1238<F: Float>(t2771: F, t4351: F, t247: F, t4344: F, t927: F, t101: F, t5915: F, t754: F, t757: F, t10960: F, t10964: F, t10967: F, t10976: F, t10980: F, t10985: F, t10991: F, t10993: F, t11582: F, t11637: F, t11664: F, t11684: F, t14564: F, t14606: F, t14625: F, t14752: F, t2: F, t328: F, t8024: F, t8028: F, t8032: F, t8034: F, t8039: F, t8043: F, t8047: F) -> F {
    let t14758 = t4351 * t2771;
    let t14761 = t247 * t927 * t4344;
    let t14765 = t101 * t5915 * t754 * t757;
    let tv4rho41 = F::new(1.4220018064581168) * t10967 - F::new(1.2654485932329695) * t10980 + t10985 + t8024 + F::new(0.41076328840066667) * t10960 + F::new(1.898172889849454) * t10964 + t10991 + F::new(0.41076328840066667) * t8028 + F::new(3.0) * t10993 + t2 * (t11582 + t11637 + t11664 + t11684 + t14564 + t14606 + t14625 + t14752) * t328 - F::new(0.22820182688925925) * t14758 + t10976 + F::new(0.4740006021527056) * t14761 + t8047 - t8039 + F::new(1.232289865202) * t14765 - F::new(3.796345779698908) * t8032 - F::new(0.6846054806677778) * t8034 + t8043;
    tv4rho41
}
