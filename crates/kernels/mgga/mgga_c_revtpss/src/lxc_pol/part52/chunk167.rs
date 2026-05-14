//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 167/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk167<F: Float>(t143: F, t130: F, t131: F, t72: F, t122: F, t125: F) -> (F, F, F, F, F) {
    let t680 = t143 * t143;
    let t681 = 1.0 / t680;
    let t682 = t130 * t681;
    let t684 = 1.0 / t131 * t72;
    let t685 = t122 * t125;
    (t680, t681, t682, t684, t685)
}
