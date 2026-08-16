//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 740/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk740(t2557: f64, t3787: f64, t1325: f64, t4957: f64, t806: f64, t4956: f64, t1449: f64, t2549: f64, t519: f64, t2553: f64, t3883: f64, t1475: f64, t2539: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6689 = t3787 * t2557;
    let t6690 = t1325 * t6689;
    let t6692 = t4957 * t806;
    let t6693 = t4956 * t6692;
    let t6696 = t1449 * t2549;
    let t6697 = t519 * t6696;
    let t6699 = t3883 * t2553;
    let t6700 = t519 * t6699;
    let t6702 = t1475 * t2539;
    (t6689, t6690, t6692, t6693, t6696, t6697, t6699, t6700, t6702)
}
