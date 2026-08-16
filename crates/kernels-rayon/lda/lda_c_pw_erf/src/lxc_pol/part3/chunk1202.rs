//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1202/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1202(t3416: f64, t5226: f64, t1318: f64, t1319: f64, t2000: f64, t2973: f64, t1954: f64, t4758: f64, t954: f64, t4753: f64, t5231: f64, t2967: f64, t3589: f64, t4776: f64, t811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14166 = 8.0_f64 / 15.0_f64 * t3416 * t5226;
    let t14170 = 8.0_f64 / 45.0_f64 * t1318 * t1319 * t2000 * t2973;
    let t14174 = 8.0_f64 / 15.0_f64 * t1318 * t4758 * t1954 * t954;
    let t14176 = 8.0_f64 / 9.0_f64 * t4753 * t5231;
    let t14178 = 8.0_f64 / 9.0_f64 * t3416 * t5231;
    let t14183 = 64.0_f64 / 81.0_f64 * t1318 * t4776 * t811 * t3589 * t2967;
    (t14166, t14170, t14174, t14176, t14178, t14183)
}
