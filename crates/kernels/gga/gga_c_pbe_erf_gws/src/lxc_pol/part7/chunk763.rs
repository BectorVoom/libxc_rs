//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 763/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk763<F: Float>(t339: F, t6738: F, t338: F, t376: F, t2271: F, t2365: F, t822: F, t833: F, t2367: F, t2397: F, t2402: F, t892: F, t2373: F, t2384: F, t2401: F, t2408: F, t335: F, t6130: F, t6135: F, t6140: F, t6145: F, t6151: F, t6156: F, t6160: F, t6164: F, t6170: F, t6173: F, t6175: F, t6726: F, t6731: F, t827: F) -> (F, F, F, F, F, F) {
    let t6739 = t339 * t6738;
    let t6741 = t338 * t6739 * t376;
    let t6744 = t2271 * t2365;
    let t6745 = t822 * t6744;
    let t6746 = t6745 * t833;
    let t6748 = t2367 * t2397;
    let t6751 = t338 * t892 * t2402;
    let t6754 = -t335 * t6130 / 16.0 - t827 * t6135 / 8.0 - t2408 * t6140 / 8.0 + t827 * t6145 / 16.0 + 3.0 / 16.0 * t827 * t6151 + 7.0 / 96.0 * t6156 + t6160 * t6164 / 48.0 - t2384 * t2373 / 16.0 - t335 * t6170 / 32.0 - 7.0 / 48.0 * t6173 + 7.0 / 96.0 * t6175 - t335 * t6726 / 96.0 - t6731 + t335 * t6741 / 96.0 - 7.0 / 96.0 * t6746 - 7.0 / 48.0 * t6748 + 3.0 / 16.0 * t2401 * t6751;
    (t6739, t6741, t6744, t6745, t6751, t6754)
}
