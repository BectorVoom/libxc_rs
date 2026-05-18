//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1390/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1390<F: Float>(t52696: F, t54331: F, t55596: F, t55603: F, t57171: F, t57174: F, t57176: F, t57179: F, t57182: F, t57184: F, t57186: F, t57188: F, t57191: F) -> F {
    let t58752 = -t57171 / F::new(384.0) - t57174 / F::new(48.0) + F::new(7.0) / F::new(576.0) * t57176 + t57179 / F::new(8.0) - t55596 - t54331 - t52696 - F::new(7.0) / F::new(192.0) * t57182 - t57184 / F::new(8.0) - t57186 / F::new(8.0) - F::new(35.0) / F::new(288.0) * t57188 - t57191 / F::new(48.0) - t55603;
    t58752
}
