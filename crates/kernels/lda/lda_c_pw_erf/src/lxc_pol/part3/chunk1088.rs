//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1088/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1088<F: Float>(t12728: F, t2114: F, t4039: F, t9680: F, t9711: F, t9714: F, t9718: F, t12718: F, t12719: F, t12720: F, t12721: F, t12722: F, t12724: F, t12726: F) -> (F, F, F, F, F, F, F) {
    let t12729 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t12728;
    let t12731 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2114 * t4039;
    let t12732 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9680;
    let t12733 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t9711;
    let t12734 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t9714;
    let t12735 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9718;
    let t12736 = -t12718 + t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - t12729 + t12731 + t12732 - t12733 - t12734 + t12735;
    (t12729, t12731, t12732, t12733, t12734, t12735, t12736)
}
