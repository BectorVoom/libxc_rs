//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 608/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk608(t1241: f64, t1249: f64, t1259: f64, t1274: f64, t1280: f64, t2215: f64, t2227: f64, t2694: f64, t2698: f64, t2701: f64, t2704: f64, t2708: f64, t2712: f64, t2715: f64, t360: f64, t63: f64) -> f64 {
    let t2718 = -t1241 + t2694 + t1249 + t2698 - t2701 + t1259 + t2215 / 3.0_f64 + 3.0_f64 / 2.0_f64 * t360 * t2704 - t360 * t2708 / 2.0_f64 + t1274 + 1.46904_f64 * t2227 + t1280 + 5.87616_f64 * t63 * t2712 - 1.46904_f64 * t63 * t2715;
    t2718
}
