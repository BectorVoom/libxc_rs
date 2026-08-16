//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk989<F: Float>(t10829: F, t10874: F, t5783: F, t6154: F, t8759: F, t8771: F, t8774: F, t8785: F, t8789: F, t8793: F, t8805: F, t8808: F, t8812: F, t8813: F, t8816: F, t8821: F, t9121: F) -> F {
    let t11530 = F::cast_from(6.0_f64) * t6154 * t9121 - F::cast_from(9.0_f64) * t5783 * t10874 - F::cast_from(9.0_f64) * t5783 * t10829 + t8759 + F::cast_from(0.17961351015381913_f64) * t8771 + t8774 - F::cast_from(0.01197423401025461_f64) * t8785 - F::cast_from(0.03592270203076383_f64) * t8789 - F::cast_from(0.03592270203076383_f64) * t8793 - t8805 - F::cast_from(1.370765728342244e-05_f64) * t8808 - t8812 + F::cast_from(0.019957056683757683_f64) * t8813 + F::cast_from(0.11974234010254609_f64) * t8816 + t8821;
    t11530
}
