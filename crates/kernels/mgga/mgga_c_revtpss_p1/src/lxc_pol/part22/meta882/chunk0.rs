//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3055/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3055<F: Float>(t10504: F, t136: F, t2457: F, t4533: F, t14481: F, t2782: F, t861: F, t11050: F, t14987: F, t14473: F, t9303: F, t41017: F, t4481: F) -> (F, F, F, F, F) {
    let t51726 = t10504 * t4533 * t136 * t2457;
    let t51729 = t2782 * t861 * t14481;
    let t51731 = t14987 * t11050;
    let t51733 = t9303 * t14473;
    let t51739 = t41017 * t4481;
    (t51726, t51729, t51731, t51733, t51739)
}
