//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1204/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1204<F: Float>(t54038: F, t54094: F, t55452: F, t55460: F, t55467: F, t56910: F, t56912: F, t56914: F, t56917: F, t56920: F, t56922: F, t56924: F, t56926: F, t55473: F, t55480: F, t55482: F, t56929: F, t56931: F, t56933: F, t56935: F, t56938: F, t56940: F, t56943: F, t56945: F, t56947: F, t56949: F) -> (F, F) {
    let t58619 = t54038 + t56910 / 24.0 - t55452 + t56912 / 96.0 + t56914 / 12.0 + t55460 + t56917 / 24.0 - t56920 / 48.0 + 7.0 / 576.0 * t56922 + t55467 + 35.0 / 108.0 * t54094 + t56924 / 96.0 - t56926 / 384.0;
    let t58630 = t56929 / 48.0 + t56931 / 48.0 + t56933 / 48.0 - 7.0 / 576.0 * t56935 + t56938 / 8.0 + t55473 - 7.0 / 144.0 * t56940 - t56943 / 6.0 + t55480 + t55482 - t56945 / 48.0 - 5.0 / 32.0 * t56947 - t56949 / 24.0;
    (t58619, t58630)
}
