//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1126/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1126<F: Float>(t14791: F, t3066: F, t51807: F, t53874: F, t53876: F, t53878: F, t53880: F, t53884: F, t53886: F, t53889: F, t53892: F, t53894: F, t53897: F, t53904: F, t53906: F, t53910: F, t8647: F, t9283: F) -> (F,) {
    let t53912 = t53874 - t53876 / 256.0 - t53878 / 24.0 + t53880 / 16.0 + t53884 / 96.0 + 119.0 / 6912.0 * t53886 + t53889 / 96.0 - t53892 / 48.0 - t53894 / 96.0 - t53897 - t3066 * t9283 * t14791 * t8647 / 8.0 + t53904 / 96.0 + t53906 / 96.0 + 7.0 / 4608.0 * t51807 - t53910 / 96.0;
    (t53912,)
}
