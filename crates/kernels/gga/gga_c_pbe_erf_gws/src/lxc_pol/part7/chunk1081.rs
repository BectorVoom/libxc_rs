//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1081/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1081<F: Float>(t6222: F, t6484: F, t20851: F, t21614: F, t21616: F, t21627: F, t21632: F, t21635: F, t21640: F, t21647: F, t21651: F, t2337: F, t3235: F, t6110: F, t6282: F, t902: F, t905: F, t9425: F) -> (F, F) {
    let t21652 = t6484 * t6222;
    let t21653 = 7.0 / 12.0 * t21652;
    let t21658 = t21614 - t21616 + t902 * t905 * t2337 * t6110 / 512.0 + t21627 + t21632 - t21635 + t21640 + t21647 - t21651 - t21653 - 3.0 / 64.0 * t9425 * t3235 * t6282 * t20851;
    (t21653, t21658)
}
