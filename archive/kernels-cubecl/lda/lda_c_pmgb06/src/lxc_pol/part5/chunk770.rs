//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 770/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk770<F: Float>(t3391: F, t6641: F, t6645: F, t6648: F, t6650: F, t6652: F, t6654: F, t6657: F, t6677: F, t6690: F, t6692: F, t6694: F, t6707: F, t6709: F, t6711: F) -> F {
    let t7221 = t6641 + t6645 - t6648 + t6650 + t6652 + t6654 + t6657 + t6677 + t6690 + t6692 + t6694 + t6707 + t6709 + t6711 + t3391;
    t7221
}
