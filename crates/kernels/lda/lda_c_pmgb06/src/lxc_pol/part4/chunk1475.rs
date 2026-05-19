//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1475/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1475<F: Float>(t11200: F, t11586: F, t11639: F, t1227: F, t1233: F, t1234: F, t1289: F, t1316: F, t14593: F, t14633: F, t18568: F, t18903: F, t18911: F, t18915: F, t18926: F, t18940: F, t18973: F, t18985: F, t19022: F, t19041: F, t2180: F, t2181: F, t2733: F, t312: F, t329: F, t346: F, t384: F, t388: F, t4021: F, t4044: F, t4232: F, t4234: F, t4360: F, t4394: F, t5583: F, t5999: F, t6006: F, t6007: F, t6018: F, t7089: F, t77: F, t783: F, t787: F, t790: F) -> F {
    let t19045 = F::new(6.0) * t2180 * t2733 * t4021 + F::new(3.0) * t329 * t77 * t18568 + F::new(3.0) * t1316 * t790 * t787 * t1227 + F::new(3.0) * t1316 * t388 * t18903 - F::new(24.0) * t6018 * t14633 + F::cast_from(0.19816831758676853_f64) * t18911 + F::cast_from(0.001355981270834723_f64) * t18915 + F::new(24.0) * t11200 * t5999 + F::new(12.0) * t2180 * t2181 * t4394 - F::new(6.0) * t6018 * t4232 * t783 * t1234 - F::new(6.0) * t6006 * t18926 * t783 * t4044 - F::new(6.0) * t5583 * t14593 + F::new(24.0) * t1233 * t11639 * t4360 + F::new(2.0) * t346 * t7089 * t384 - F::new(6.0) * t18940 * t4234 - F::new(3.0) * t5583 * t4232 * t783 * t1227 + F::new(2.0) * t6006 * t6007 * t783 * t1289 + F::new(12.0) * t6018 * t11586 + (t18973 + t18985 + t19022 + t19041) * t312;
    t19045
}
