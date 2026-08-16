//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 562/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk562(t1386: f64, t2948: f64, t439: f64, t1536: f64, t477: f64, t1385: f64, t1629: f64, t454: f64, t1436: f64, t464: f64, t1526: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2949 = t2948 * t1386;
    let t2951 = 2.0_f64 / 15.0_f64 * t439 * t2949;
    let t2952 = t1536 * t477;
    let t2953 = t1385 * t2952;
    let t2955 = t439 * t2953 / 15.0_f64;
    let t2956 = t454 * t1629;
    let t2957 = t1385 * t2956;
    let t2959 = t439 * t2957 / 15.0_f64;
    let t2960 = t1436 * t464;
    let t2961 = t1526 * t477;
    let t2962 = t2960 * t2961;
    let t2964 = t439 * t2962 / 9.0_f64;
    (t2949, t2951, t2952, t2953, t2955, t2956, t2957, t2959, t2960, t2961, t2962, t2964)
}
