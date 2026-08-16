//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 828/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk828(t1553: f64, t776: f64, t405: f64, t247: f64, t4713: f64, t251: f64, t2252: f64, t652: f64, t256: f64, t19: f64, t1904: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5782 = t776 * t1553;
    let t5783 = t405 * t5782;
    let t5787 = t4713 * t247;
    let t5788 = t5787 * t251;
    let t5791 = t2252 * t652;
    let t5793 = 2.0_f64 / 3.0_f64 * t5791 * t256;
    let t5794 = t1904 * t19;
    let t5795 = t5794 * t644;
    (t5782, t5783, t5787, t5788, t5791, t5793, t5794, t5795)
}
