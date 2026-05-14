//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 999/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk999<F: Float>(t10419: F, t3564: F, t12711: F, t2741: F, t10743: F, t186: F, t220: F, t47638: F, t616: F, t3451: F, t40402: F, t10969: F, t12617: F, t2612: F, t12620: F, t12623: F) -> (F, F, F, F, F, F, F, F, F) {
    let t48067 = 8.0 / 5.0 * t10419 * t3564;
    let t48069 = 16.0 / 15.0 * t2741 * t12711;
    let t48071 = 16.0 / 15.0 * t10743 * t12711;
    let t48076 = -4.0 / 15.0 * t616 * t186 * t220 * t47638;
    let t48078 = 16.0 / 5.0 * t40402 * t3451;
    let t48080 = 8.0 / 5.0 * t10969 * t3451;
    let t48082 = 16.0 / 15.0 * t2612 * t12617;
    let t48084 = 32.0 / 15.0 * t2612 * t12620;
    let t48086 = 16.0 / 9.0 * t2612 * t12623;
    (t48067, t48069, t48071, t48076, t48078, t48080, t48082, t48084, t48086)
}
