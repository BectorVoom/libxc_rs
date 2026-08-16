//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1093/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1093(t12781: f64, t1325: f64, t5291: f64, t12756: f64, t12757: f64, t12758: f64, t12759: f64, t12760: f64, t12761: f64, t12762: f64, t12763: f64, t12764: f64, t12770: f64, t12775: f64, t12780: f64) -> (f64, f64) {
    let t12783 = t1325 * t12781 * t5291;
    let t12784 = 32.0_f64 / 15.0_f64 * t12783;
    let t12785 = t12756 - t12757 - t12758 - t12759 + t12760 - t12761 - t12762 - t12763 + t12764 - t12770 - t12775 + t12780 + t12784;
    (t12784, t12785)
}
