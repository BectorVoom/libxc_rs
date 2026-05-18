//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 587/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk587<F: Float>(t338: F, t353: F, t4436: F, t2200: F, t329: F, t340: F, t847: F, t2231: F, t892: F, t2366: F, t2387: F, t833: F) -> (F, F, F, F, F, F) {
    let t4438 = t338 * t353 * t4436;
    let t4442 = t329 * t2200 * t340;
    let t4443 = t4442 * t847;
    let t4446 = t338 * t892 * t2231;
    let t4453 = t2387 * t2366;
    let t4454 = t4453 * t833;
    (t4438, t4442, t4443, t4446, t4453, t4454)
}
