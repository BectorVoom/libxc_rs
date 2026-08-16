//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1283/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1283(t2466: f64, t3223: f64, t161: f64, t489: f64, t6905: f64, t10711: f64, t10714: f64, t16840: f64, t16841: f64, t16843: f64, t16847: f64, t16849: f64, t16852: f64, t16855: f64, t16860: f64, t16862: f64, t16864: f64, t16865: f64) -> (f64, f64, f64) {
    let t16866 = t3223 * t2466;
    let t16867 = 2.0_f64 / 405.0_f64 * t16866;
    let t16869 = t161 * t489 * t6905;
    let t16870 = 2.0_f64 / 45.0_f64 * t16869;
    let t16871 = -t16840 - t16841 + t10711 + t10714 - t16843 + t16847 + t16849 + t16852 - t16855 - t16860 - t16862 - t16864 + t16865 - t16867 - t16870;
    (t16867, t16870, t16871)
}
