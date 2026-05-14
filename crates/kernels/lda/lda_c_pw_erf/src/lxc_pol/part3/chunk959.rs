//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 959/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk959<F: Float>(t9629: F, t9645: F, t9647: F, t2127: F, t5069: F, t2131: F, t211: F, t5030: F, t514: F, t2114: F, t4039: F, t9680: F, t9711: F, t9714: F, t9718: F, t12718: F, t12719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12720 = 8.0 / 15.0 * t9629;
    let t12721 = 16.0 / 135.0 * t9645;
    let t12722 = 16.0 / 15.0 * t9647;
    let t12723 = t5069 * t2127;
    let t12724 = 16.0 / 15.0 * t12723;
    let t12726 = 8.0 / 5.0 * t5069 * t2131;
    let t12728 = t211 * t514 * t5030;
    let t12729 = 4.0 / 15.0 * t12728;
    let t12731 = 4.0 / 5.0 * t2114 * t4039;
    let t12732 = 16.0 / 45.0 * t9680;
    let t12733 = 8.0 / 45.0 * t9711;
    let t12734 = 16.0 / 45.0 * t9714;
    let t12735 = 8.0 / 27.0 * t9718;
    let t12736 = -t12718 + t12719 + t12720 - t12721 - t12722 + t12724 + t12726 - t12729 + t12731 + t12732 - t12733 - t12734 + t12735;
    (t12720, t12721, t12722, t12724, t12726, t12729, t12731, t12732, t12733, t12734, t12735, t12736)
}
