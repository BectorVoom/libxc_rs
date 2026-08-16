//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 663/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk663(t1179: f64, t206: f64, t4068: f64, t2951: f64, t2955: f64, t2959: f64, t2964: f64, t2968: f64, t2973: f64, t2976: f64, t2978: f64, t2982: f64, t2986: f64, t2990: f64, t2995: f64, t2997: f64, t2999: f64, t3001: f64, t3007: f64) -> (f64, f64) {
    let t4070 = 0.001515438175925926_f64 * t206 * t1179 * t4068;
    let t4071 = -t2951 - t2955 - t2959 - t2964 + t2968 - t2973 - t2976 - t2978 - t2982 - t2986 - t2990 - t2995 + t2997 - t2999 - t3001 + t3007 + t4070;
    (t4070, t4071)
}
