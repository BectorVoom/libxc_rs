//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1277/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1277<F: Float>(t14617: F, t51581: F, t14135: F, t3039: F, t14138: F, t20154: F, t3067: F, t4155: F, t938: F, t2376: F, t26617: F, t810: F) -> (F, F, F, F) {
    let t53772 = t51581 * t14617;
    let t53774 = t3039 * t14135;
    let t53775 = t53774 * t14138;
    let t53779 = t20154 * t3067 * t4155 * t938;
    let t53784 = t26617 * t2376 * t4155 * t810;
    (t53772, t53775, t53779, t53784)
}
