//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1266/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1266(t101: f64, t754: f64, t757: f64, t8019: f64, t10967: f64, t10976: f64, t10980: f64, t10985: f64, t10991: f64, t14758: f64, t14761: f64, t14773: f64, t14776: f64, t19055: f64, t19063: f64, t19076: f64, t19092: f64, t2: f64, t21305: f64, t21619: f64, t21648: f64, t21699: f64, t22123: f64, t22146: f64, t22243: f64, t328: f64, t8032: f64, t8034: f64, t8039: f64, t8043: f64, t8047: f64) -> f64 {
    let t22251 = t101 * t8019 * t754 * t757;
    let tv4rho43 = 3.0_f64 * t14776 + 0.41076328840066667_f64 * t19055 + t10985 - 3.796345779698908_f64 * t10980 - 0.6846054806677778_f64 * t14758 - 1.2654485932329695_f64 * t8032 - t8039 + t8043 - 0.22820182688925925_f64 * t8034 + t8047 + t19076 + t2 * (t19092 + t21305 + t21619 + t21648 + t21699 + t22123 + t22146 + t22243) * t328 + 0.41076328840066667_f64 * t22251 + 1.232289865202_f64 * t14773 + 1.898172889849454_f64 * t19063 + t10991 + 1.4220018064581168_f64 * t14761 + 0.4740006021527056_f64 * t10967 + t10976;
    tv4rho43
}
