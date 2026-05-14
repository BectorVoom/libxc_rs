//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 741/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk741<F: Float>(t118: F, t2174: F, t4622: F, t4624: F, t4626: F, t4628: F, t4630: F, t4684: F, t4721: F, t4723: F, t4725: F, t4727: F, t4730: F, t4734: F, t4735: F, t4738: F, t4739: F) -> (F, F) {
    let t5627 = t2174 * t118;
    let t5629 = t4622 + t4624 + t4626 + t4628 + t4630 + t4684 - t4721 - t4723 + t4725 - t4727 - t4730 - t4734 - t4735 - t4738 - t4739;
    (t5627, t5629)
}
