//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 953/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk953(t197: f64, t3892: f64, t3518: f64, t2120: f64, t3550: f64, t3553: f64, t795: f64, t4505: f64, t668: f64, t3667: f64, t573: f64, t3437: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12046 = t2120 * t3550;
    let t12047 = 8.0_f64 / 45.0_f64 * t12046;
    let t12050 = t795 * t3553;
    let t12051 = 4.0_f64 / 45.0_f64 * t12050;
    let t12064 = t4505 * t668;
    let t12071 = t573 * t3667;
    let t12083 = t822 * t3437;
    (t12031, t12047, t12051, t12064, t12071, t12083)
}
