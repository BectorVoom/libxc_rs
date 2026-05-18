//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1380/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1380<F: Float>(t55473: F, t55480: F, t55482: F, t56929: F, t56931: F, t56933: F, t56935: F, t56938: F, t56940: F, t56943: F, t56945: F, t56947: F, t56949: F) -> F {
    let t58630 = t56929 / F::new(48.0) + t56931 / F::new(48.0) + t56933 / F::new(48.0) - F::new(7.0) / F::new(576.0) * t56935 + t56938 / F::new(8.0) + t55473 - F::new(7.0) / F::new(144.0) * t56940 - t56943 / F::new(6.0) + t55480 + t55482 - t56945 / F::new(48.0) - F::new(5.0) / F::new(32.0) * t56947 - t56949 / F::new(24.0);
    t58630
}
