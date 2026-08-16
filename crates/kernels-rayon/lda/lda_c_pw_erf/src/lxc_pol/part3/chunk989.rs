//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk989(t10829: f64, t10874: f64, t5783: f64, t6154: f64, t8759: f64, t8771: f64, t8774: f64, t8785: f64, t8789: f64, t8793: f64, t8805: f64, t8808: f64, t8812: f64, t8813: f64, t8816: f64, t8821: f64, t9121: f64) -> f64 {
    let t11530 = 6.0_f64 * t6154 * t9121 - 9.0_f64 * t5783 * t10874 - 9.0_f64 * t5783 * t10829 + t8759 + 0.17961351015381913_f64 * t8771 + t8774 - 0.01197423401025461_f64 * t8785 - 0.03592270203076383_f64 * t8789 - 0.03592270203076383_f64 * t8793 - t8805 - 1.370765728342244e-05_f64 * t8808 - t8812 + 0.019957056683757683_f64 * t8813 + 0.11974234010254609_f64 * t8816 + t8821;
    t11530
}
