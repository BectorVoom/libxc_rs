//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 440/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk440(t5: f64, t1905: f64, t1986: f64, t2038: f64, t2114: f64, t107: f64, t410: f64, t902: f64, t1068: f64, t760: f64, t1: f64, t9: f64, t332: f64, t395: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t2116 = t1905 + t1986 + t2038 + t2114;
    let t2122 = t107 * t410 * t902;
    let t2125 = t1068 * t760;
    let t2128 = t9 * t1;
    let t2132 = piecewise3(t6, 0.0_f64, 4.0_f64 / 9.0_f64 * t2125 * t332 + 8.0_f64 / 3.0_f64 * t2128 * t395);
    (t2116, t2122, t2125, t2132)
}
