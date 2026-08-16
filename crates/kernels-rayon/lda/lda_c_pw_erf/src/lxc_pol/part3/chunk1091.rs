//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1091/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1091(t9939: f64, t9941: f64, t9944: f64, t9947: f64, t9949: f64, t9953: f64, t9973: f64, t9975: f64, t9977: f64, t1440: f64, t3675: f64, t1325: f64, t1392: f64, t494: f64, t806: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12756 = 4.0_f64 / 45.0_f64 * t9939;
    let t12757 = 16.0_f64 / 45.0_f64 * t9941;
    let t12758 = 8.0_f64 / 45.0_f64 * t9944;
    let t12759 = 16.0_f64 / 135.0_f64 * t9947;
    let t12760 = 8.0_f64 / 45.0_f64 * t9949;
    let t12761 = 4.0_f64 / 15.0_f64 * t9953;
    let t12762 = 4.0_f64 / 15.0_f64 * t9973;
    let t12763 = 4.0_f64 / 15.0_f64 * t9975;
    let t12764 = 8.0_f64 / 15.0_f64 * t9977;
    let t12765 = t1440 * t3675;
    let t12770 = 24.0_f64 / 5.0_f64 * t1325 * t12765 * t806 * t1392 * t494;
    (t12756, t12757, t12758, t12759, t12760, t12761, t12762, t12763, t12764, t12765, t12770)
}
