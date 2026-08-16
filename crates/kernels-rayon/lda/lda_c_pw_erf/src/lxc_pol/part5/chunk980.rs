//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 980/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk980(t1729: f64, t5782: f64, t140: f64, t6126: f64, t159: f64, t1904: f64, t285: f64, t39: f64, t1125: f64, t763: f64, t133: f64, t1844: f64, t474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    let t14515 = t39 * t1904 * t159 * t285;
    let t14516 = 0.004067943812504169_f64 * t14515;
    let t14581 = t1125 * t763;
    let t14582 = t133 * t14581;
    let t14584 = t474 * t1844;
    (t14488, t14491, t14516, t14581, t14582, t14584)
}
