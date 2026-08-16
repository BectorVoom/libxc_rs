//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1475/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1475(t11200: f64, t11586: f64, t11639: f64, t1227: f64, t1233: f64, t1234: f64, t1289: f64, t1316: f64, t14593: f64, t14633: f64, t18568: f64, t18903: f64, t18911: f64, t18915: f64, t18926: f64, t18940: f64, t18973: f64, t18985: f64, t19022: f64, t19041: f64, t2180: f64, t2181: f64, t2733: f64, t312: f64, t329: f64, t346: f64, t384: f64, t388: f64, t4021: f64, t4044: f64, t4232: f64, t4234: f64, t4360: f64, t4394: f64, t5583: f64, t5999: f64, t6006: f64, t6007: f64, t6018: f64, t7089: f64, t77: f64, t783: f64, t787: f64, t790: f64) -> f64 {
    let t19045 = 6.0_f64 * t2180 * t2733 * t4021 + 3.0_f64 * t329 * t77 * t18568 + 3.0_f64 * t1316 * t790 * t787 * t1227 + 3.0_f64 * t1316 * t388 * t18903 - 24.0_f64 * t6018 * t14633 + 0.19816831758676853_f64 * t18911 + 0.001355981270834723_f64 * t18915 + 24.0_f64 * t11200 * t5999 + 12.0_f64 * t2180 * t2181 * t4394 - 6.0_f64 * t6018 * t4232 * t783 * t1234 - 6.0_f64 * t6006 * t18926 * t783 * t4044 - 6.0_f64 * t5583 * t14593 + 24.0_f64 * t1233 * t11639 * t4360 + 2.0_f64 * t346 * t7089 * t384 - 6.0_f64 * t18940 * t4234 - 3.0_f64 * t5583 * t4232 * t783 * t1227 + 2.0_f64 * t6006 * t6007 * t783 * t1289 + 12.0_f64 * t6018 * t11586 + (t18973 + t18985 + t19022 + t19041) * t312;
    t19045
}
