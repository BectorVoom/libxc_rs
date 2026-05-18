//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 711/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk711<F: Float>(t3455: F, t582: F, t185: F, t3562: F, t626: F, t3553: F, t3410: F, t5125: F, t1820: F, t3544: F, t401: F, t3547: F) -> (F, F, F, F, F, F, F, F) {
    let t10485 = t582 * t3455;
    let t10486 = t185 * t10485;
    let t10500 = t3562 * t626;
    let t10505 = t3553 * t626;
    let t10510 = t5125 * t3410;
    let t10511 = t1820 * t10510;
    let t10517 = t401 * t3544;
    let t10519 = t401 * t3547;
    (t10485, t10486, t10500, t10505, t10510, t10511, t10517, t10519)
}
