//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 575/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk575<F: Float>(t4422: F, t824: F, t822: F, t833: F, t2387: F, t2391: F, t2242: F, t941: F, t2220: F, t338: F, t845: F, t376: F, t4379: F, t353: F, t2200: F, t329: F, t340: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4423 = t824 * t4422;
    let t4424 = t822 * t4423;
    let t4425 = t4424 * t833;
    let t4427 = t2387 * t2391;
    let t4430 = t2242 * t941;
    let t4433 = t338 * t2220 * t845;
    let t4436 = t376 * t4379;
    let t4438 = t338 * t353 * t4436;
    let t4442 = t329 * t2200 * t340;
    (t4423, t4424, t4425, t4427, t4430, t4433, t4436, t4438, t4442)
}
