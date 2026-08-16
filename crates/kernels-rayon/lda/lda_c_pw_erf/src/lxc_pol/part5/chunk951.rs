//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 951/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk951(t3476: f64, t521: f64, t1458: f64, t3518: f64, t1245: f64, t537: f64, t188: f64, t1: f64, t1184: f64, t2071: f64, t548: f64, t3604: f64, t5165: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11867 = t521 * t3476;
    let t11871 = t1458 * t3518;
    let t11875 = t537 * t1245;
    let t11879 = t188 * t1245;
    let t11898 = t1 * t1184;
    let t11900 = t548 * t11898 * t2071;
    let t11907 = t5165 * t3604;
    (t11867, t11871, t11875, t11879, t11898, t11900, t11907)
}
