//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1087/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1087<F: Float>(t1513: F, t2134: F, t9627: F, t9629: F, t9645: F, t9647: F, t2127: F, t5069: F, t2131: F, t211: F, t5030: F, t514: F) -> (F, F, F, F, F, F, F, F) {
    let t12717 = t1513 * t2134;
    let t12718 = F::new(8.0) / F::new(15.0) * t12717;
    let t12719 = F::new(8.0) / F::new(15.0) * t9627;
    let t12720 = F::new(8.0) / F::new(15.0) * t9629;
    let t12721 = F::new(16.0) / F::new(135.0) * t9645;
    let t12722 = F::new(16.0) / F::new(15.0) * t9647;
    let t12723 = t5069 * t2127;
    let t12724 = F::new(16.0) / F::new(15.0) * t12723;
    let t12726 = F::new(8.0) / F::new(5.0) * t5069 * t2131;
    let t12728 = t211 * t514 * t5030;
    (t12718, t12719, t12720, t12721, t12722, t12724, t12726, t12728)
}
