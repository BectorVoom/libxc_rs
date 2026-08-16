//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2150/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2150<F: Float>(t22685: F, t22881: F, t6330: F, t6637: F, t22893: F, t28142: F, t80681: F, t2006: F, t6387: F, t28143: F, t80727: F, t6414: F) -> (F, F, F, F, F) {
    let t97158 = t22685 * t6637 * t22881 * t6330;
    let t97161 = t80681 * t22893 * t28142;
    let t97172 = t2006 * t6387;
    let t97179 = t80727 * t28143;
    let t97181 = t2006 * t6414;
    (t97158, t97161, t97172, t97179, t97181)
}
