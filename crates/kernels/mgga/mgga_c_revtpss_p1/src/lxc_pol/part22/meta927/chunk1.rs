//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3152/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3152<F: Float>(t12702: F, t17350: F, t1263: F, t372: F, t5284: F, t13148: F, t56878: F, t17728: F, t460: F, t489: F, t17261: F, t17373: F) -> (F, F, F, F, F) {
    let t56977 = t12702 * t17350;
    let t56981 = t372 * t1263 * t5284;
    let t56997 = t13148 * t56878;
    let t57005 = t460 * t489 * t17728;
    let t57021 = t17261 * t17373;
    (t56977, t56981, t56997, t57005, t57021)
}
