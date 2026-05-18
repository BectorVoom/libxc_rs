//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1266/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1266<F: Float>(t101: F, t754: F, t757: F, t8019: F, t10967: F, t10976: F, t10980: F, t10985: F, t10991: F, t14758: F, t14761: F, t14773: F, t14776: F, t19055: F, t19063: F, t19076: F, t19092: F, t2: F, t21305: F, t21619: F, t21648: F, t21699: F, t22123: F, t22146: F, t22243: F, t328: F, t8032: F, t8034: F, t8039: F, t8043: F, t8047: F) -> F {
    let t22251 = t101 * t8019 * t754 * t757;
    let tv4rho43 = F::new(3.0) * t14776 + F::new(0.41076328840066667) * t19055 + t10985 - F::new(3.796345779698908) * t10980 - F::new(0.6846054806677778) * t14758 - F::new(1.2654485932329695) * t8032 - t8039 + t8043 - F::new(0.22820182688925925) * t8034 + t8047 + t19076 + t2 * (t19092 + t21305 + t21619 + t21648 + t21699 + t22123 + t22146 + t22243) * t328 + F::new(0.41076328840066667) * t22251 + F::new(1.232289865202) * t14773 + F::new(1.898172889849454) * t19063 + t10991 + F::new(1.4220018064581168) * t14761 + F::new(0.4740006021527056) * t10967 + t10976;
    tv4rho43
}
