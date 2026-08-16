//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1221/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1221<F: Float>(t21652: F, t20851: F, t21614: F, t21616: F, t21627: F, t21632: F, t21635: F, t21640: F, t21647: F, t21651: F, t2337: F, t3235: F, t6110: F, t6282: F, t902: F, t905: F, t9425: F) -> (F, F) {
    let t21653 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t21652;
    let t21658 = t21614 - t21616 + t902 * t905 * t2337 * t6110 / F::cast_from(512.0_f64) + t21627 + t21632 - t21635 + t21640 + t21647 - t21651 - t21653 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t9425 * t3235 * t6282 * t20851;
    (t21653, t21658)
}
