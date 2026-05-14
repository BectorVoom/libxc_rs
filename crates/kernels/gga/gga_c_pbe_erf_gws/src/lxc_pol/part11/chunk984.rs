//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 984/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk984<F: Float>(t41069: F, t41074: F, t12550: F, t2615: F, t47400: F, t587: F, t590: F, t591: F, t10848: F, t3531: F, t12556: F, t12634: F, t5218: F, t7495: F, t12804: F, t24848: F) -> (F, F, F, F, F, F, F, F) {
    let t47811 = 64.0 / 15.0 * t41069;
    let t47812 = 32.0 / 15.0 * t41074;
    let t47814 = 16.0 / 45.0 * t2615 * t12550;
    let t47818 = 4.0 / 45.0 * t587 * t590 * t591 * t47400;
    let t47820 = 8.0 / 9.0 * t10848 * t3531;
    let t47822 = 128.0 / 81.0 * t2615 * t12556;
    let t47825 = 32.0 / 15.0 * t5218 * t7495 * t12634;
    let t47828 = 32.0 / 9.0 * t5218 * t24848 * t12804;
    (t47811, t47812, t47814, t47818, t47820, t47822, t47825, t47828)
}
