//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 699/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk699(t493: f64, t6220: f64, t2134: f64, t795: f64, t2463: f64, t656: f64, t2402: f64, t568: f64, t1976: f64, t739: f64, t4829: f64, t1325: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6221 = t493 * t6220;
    let t6222 = 8.0_f64 / 45.0_f64 * t6221;
    let t6223 = t795 * t2134;
    let t6224 = 8.0_f64 / 45.0_f64 * t6223;
    let t6225 = t2463 * t656;
    let t6227 = t2402 * t568;
    let t6228 = 8.0_f64 / 45.0_f64 * t6227;
    let t6229 = t1976 * t739;
    let t6230 = t4829 * t6229;
    let t6232 = 16.0_f64 / 45.0_f64 * t1325 * t6230;
    (t6221, t6222, t6223, t6224, t6225, t6227, t6228, t6229, t6230, t6232)
}
