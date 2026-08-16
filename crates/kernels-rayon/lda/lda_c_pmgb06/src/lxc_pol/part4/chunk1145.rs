//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1145/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1145(t1101: f64, t2396: f64, t1108: f64, t1105: f64, t11164: f64, t11166: f64, t11168: f64, t11171: f64, t11174: f64, t11177: f64, t11178: f64, t11180: f64, t8837: f64, t8841: f64, t8842: f64, t8844: f64, t8846: f64, t8853: f64, t9037: f64) -> f64 {
    let t15045 = t1101 * t2396;
    let t15047 = t1108 * t2396;
    let t15054 = t1105 * t2396;
    let t15056 = -2.3392894490538585_f64 * t11164 - 2050.8037716432814_f64 * t11166 - 69.26343642272586_f64 * t11168 - 1.1696447245269292_f64 * t11171 + 4.0_f64 * t11174 + 2.0_f64 * t11177 - t8837 + 20.0_f64 * t15045 - 32.0_f64 * t15047 + t8841 - 1.1696447245269292_f64 * t11178 - 7.017868347161575_f64 * t11180 + 192.0_f64 * t8842 + 48.0_f64 * t8844 + 96.0_f64 * t8846 + 12.0_f64 * t15054 - t8853 + t9037;
    t15056
}
