//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1379/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1379<F: Float>(t54038: F, t54094: F, t55452: F, t55460: F, t55467: F, t56910: F, t56912: F, t56914: F, t56917: F, t56920: F, t56922: F, t56924: F, t56926: F) -> F {
    let t58619 = t54038 + t56910 / F::new(24.0) - t55452 + t56912 / F::new(96.0) + t56914 / F::new(12.0) + t55460 + t56917 / F::new(24.0) - t56920 / F::new(48.0) + F::new(7.0) / F::new(576.0) * t56922 + t55467 + F::new(35.0) / F::new(108.0) * t54094 + t56924 / F::new(96.0) - t56926 / F::new(384.0);
    t58619
}
