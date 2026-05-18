//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1399/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1399<F: Float>(t3200: F, t335: F, t338: F, t4228: F, t52582: F, t52586: F, t52589: F, t54641: F, t55090: F, t55182: F, t55904: F, t55918: F, t55936: F, t55942: F, t57668: F, t57671: F, t57674: F, t57678: F, t57685: F, t8793: F) -> F {
    let t58919 = -t57668 / F::new(12.0) + t57671 / F::new(24.0) - F::new(35.0) / F::new(216.0) * t55904 - t52582 - t55918 - t55936 - t57674 / F::new(4.0) - t55942 - t57678 / F::new(192.0) + F::new(35.0) / F::new(108.0) * t54641 - t8793 * t55090 / F::new(12.0) - t8793 * t55182 / F::new(8.0) - F::new(35.0) / F::new(432.0) * t52586 + t52589 - t335 * t338 * t3200 * t4228 / F::new(48.0) - t57685 / F::new(2.0);
    t58919
}
