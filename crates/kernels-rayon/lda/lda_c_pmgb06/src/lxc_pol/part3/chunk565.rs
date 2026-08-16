//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 565/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk565(t1381: f64, t2979: f64, t493: f64, t1485: f64, t529: f64, t1380: f64, t1586: f64, t507: f64, t1462: f64, t517: f64, t1477: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2980 = t2979 * t1381;
    let t2982 = 2.0_f64 / 15.0_f64 * t493 * t2980;
    let t2983 = t1485 * t529;
    let t2984 = t1380 * t2983;
    let t2986 = t493 * t2984 / 15.0_f64;
    let t2987 = t507 * t1586;
    let t2988 = t1380 * t2987;
    let t2990 = t493 * t2988 / 15.0_f64;
    let t2991 = t1462 * t517;
    let t2992 = t1477 * t529;
    let t2993 = t2991 * t2992;
    let t2995 = t493 * t2993 / 9.0_f64;
    (t2980, t2982, t2983, t2984, t2986, t2987, t2988, t2990, t2991, t2992, t2993, t2995)
}
