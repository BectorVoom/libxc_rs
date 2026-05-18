//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 574/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk574<F: Float>(t539: F, t933: F, t177: F, t504: F, t191: F, t1244: F, t259: F) -> (F, F, F, F) {
    let t3508 = t933 * t539;
    let t3515 = F::new(1.0) / t177 / t504;
    let t3516 = t191 * t3515;
    let t3518 = F::new(1.0) / t1244 / t259;
    (t3508, t3515, t3516, t3518)
}
