//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 510/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk510<F: Float>(t3443: F, t598: F, t186: F, t185: F, t2790: F, t997: F, t198: F, t3345: F) -> (F, F, F, F, F, F) {
    let t3444 = t598 * t3443;
    let t3445 = t186 * t3444;
    let t3447 = F::new(2.0) / F::new(15.0) * t185 * t3445;
    let t3449 = F::new(8.0) / F::new(15.0) * t2790 * t997;
    let t3450 = t198 * t3345;
    let t3451 = t186 * t3450;
    (t3444, t3445, t3447, t3449, t3450, t3451)
}
