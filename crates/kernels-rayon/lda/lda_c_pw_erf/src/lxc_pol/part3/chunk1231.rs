//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1231/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1231(t1729: f64, t1880: f64, t405: f64, t6153: f64, t5782: f64, t140: f64, t6126: f64, t10832: f64, t1872: f64, t5673: f64, t684: f64, t2765: f64, t5647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14480 = t1729 * t1880;
    let t14485 = t405 * t6153;
    let t14488 = t1729 * t5782;
    let t14491 = t6126 * t140;
    let t14500 = t10832 * t1872;
    let t14503 = t684 * t5673;
    let t14505 = t2765 * t5647;
    (t14480, t14485, t14488, t14491, t14500, t14503, t14505)
}
