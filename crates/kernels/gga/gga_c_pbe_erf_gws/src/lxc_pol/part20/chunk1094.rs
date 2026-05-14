//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1094/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1094<F: Float>(t353: F, t4183: F, t814: F, t859: F, t20154: F, t2376: F, t4155: F, t14724: F, t343: F, t361: F, t14809: F, t4414: F, t14469: F, t50943: F, t13793: F, t53229: F) -> (F, F, F, F, F, F) {
    let t53464 = t859 * t353 * t4183 * t814;
    let t53472 = t20154 * t2376 * t4155 * t814;
    let t53496 = t361 * t14724 * t343;
    let t53503 = 7.0 / 72.0 * t4414 * t14809;
    let t53508 = t50943 * t14469;
    let t53509 = 7.0 / 72.0 * t53508;
    let t53515 = t53229 * t13793;
    (t53464, t53472, t53496, t53503, t53509, t53515)
}
