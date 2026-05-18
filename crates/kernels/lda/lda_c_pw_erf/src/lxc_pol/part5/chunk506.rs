//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 506/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk506<F: Float>(t2497: F, t530: F, t186: F, t185: F, t2120: F, t786: F, t198: F, t2328: F) -> (F, F, F, F, F, F) {
    let t2498 = t530 * t2497;
    let t2499 = t186 * t2498;
    let t2501 = F::new(2.0) / F::new(15.0) * t185 * t2499;
    let t2503 = F::new(8.0) / F::new(15.0) * t2120 * t786;
    let t2504 = t198 * t2328;
    let t2505 = t186 * t2504;
    (t2498, t2499, t2501, t2503, t2504, t2505)
}
