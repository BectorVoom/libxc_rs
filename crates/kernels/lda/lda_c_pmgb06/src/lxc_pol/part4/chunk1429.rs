//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1429/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1429<F: Float>(t17590: F, t17593: F, t17597: F, t17601: F, t17604: F, t17607: F, t17610: F, t17614: F, t17616: F, t17620: F, t17624: F, t17627: F, t17631: F, t17634: F, t17637: F) -> F {
    let t18315 = t17590 + t17593 - t17597 + t17601 + t17604 - t17607 + t17610 + t17614 + t17616 + t17620 - t17624 + t17627 - t17631 + t17634 - t17637;
    t18315
}
