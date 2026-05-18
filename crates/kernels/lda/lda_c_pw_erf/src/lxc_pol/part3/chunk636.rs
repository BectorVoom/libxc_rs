//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 636/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk636<F: Float>(t3757: F, t571: F, t1472: F, t1476: F, t155: F, t573: F) -> (F, F, F, F) {
    let t3759 = F::new(4.0) / F::new(5.0) * t571 * t3757;
    let t3760 = t1472 * t1476;
    let t3761 = F::new(16.0) / F::new(45.0) * t3760;
    let t3762 = t155 * t573;
    (t3759, t3760, t3761, t3762)
}
