//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3136/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3136<F: Float>(t81635: F, t81638: F, t81641: F, t81646: F, t81649: F, t81653: F, t81656: F, t81660: F, t82119: F, t82385: F, t82386: F, t82388: F, t82391: F, t82419: F) -> F {
    let t82422 = t82385 + t82386 + t82388 + t81635 + t81638 - t81641 - t81646 - t81649 + t81653 + t81656 + t81660 + t82119 - t82391 + t82419;
    t82422
}
