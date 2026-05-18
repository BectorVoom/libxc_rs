//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1286/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1286<F: Float>(t53896: F, t3950: F, t833: F, t850: F, t9170: F, t13944: F, t2503: F, t2409: F, t28457: F, t3965: F, t14791: F, t3066: F, t51807: F, t53874: F, t53876: F, t53878: F, t53880: F, t53884: F, t53886: F, t53889: F, t53892: F, t53894: F, t8647: F, t9283: F) -> F {
    let t53897 = F::new(7.0) / F::new(72.0) * t53896;
    let t53904 = t850 * t9170 * t3950 * t833;
    let t53906 = t13944 * t2503;
    let t53910 = t3965 * t2409 * t28457;
    let t53912 = t53874 - t53876 / F::new(256.0) - t53878 / F::new(24.0) + t53880 / F::new(16.0) + t53884 / F::new(96.0) + F::new(119.0) / F::new(6912.0) * t53886 + t53889 / F::new(96.0) - t53892 / F::new(48.0) - t53894 / F::new(96.0) - t53897 - t3066 * t9283 * t14791 * t8647 / F::new(8.0) + t53904 / F::new(96.0) + t53906 / F::new(96.0) + F::new(7.0) / F::new(4608.0) * t51807 - t53910 / F::new(96.0);
    t53912
}
