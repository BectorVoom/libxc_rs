//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 903/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk903<F: Float>(t2749: F, t3479: F, t12701: F, t597: F, t1802: F, t10510: F, t7130: F, t10992: F, t2615: F, t12584: F, t211: F, t582: F, t12729: F, t12709: F, t649: F, t1017: F, t3346: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40563 = t3479 * t2749;
    let t40566 = t597 * t12701;
    let t40571 = t1802 * t12701;
    let t40604 = t7130 * t10510;
    let t40655 = t2615 * t10992;
    let t40672 = t211 * t582 * t12584;
    let t40676 = t597 * t12729;
    let t40687 = t649 * t12709;
    let t40696 = t3346 * t1017;
    (t40563, t40566, t40571, t40604, t40655, t40672, t40676, t40687, t40696)
}
