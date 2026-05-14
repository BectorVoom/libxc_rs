//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 751/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk751<F: Float>(t6212: F, t6216: F, t6221: F, t6223: F, t7460: F, t7462: F, t7464: F, t7468: F, t7472: F, t7473: F, t7477: F, t7481: F, t7483: F, t7487: F, t7491: F, t7493: F) -> (F, F, F, F, F) {
    let t7494 = 16.0 / 15.0 * t6212;
    let t7495 = 8.0 / 15.0 * t6216;
    let t7496 = 8.0 / 15.0 * t6221;
    let t7497 = 8.0 / 15.0 * t6223;
    let t7498 = -t7460 - t7462 - t7464 + t7468 + t7472 - t7473 + t7477 + t7481 - t7483 - t7487 - t7491 - t7493 + t7494 + t7495 + t7496 - t7497;
    (t7494, t7495, t7496, t7497, t7498)
}
