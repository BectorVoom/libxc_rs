//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 969/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk969<F: Float>(t40321: F, t22996: F, t47545: F, t47546: F, t47547: F, t47548: F, t47552: F, t47554: F, t47555: F, t47559: F, t47560: F, t47561: F, t40324: F, t40327: F, t40358: F, t40361: F) -> (F, F, F, F, F, F) {
    let t47562 = 32.0 / 27.0 * t40321;
    let t47563 = 0.14e-19 * t22996 - t47545 + t47546 + t47547 - t47548 + t47552 + t47554 - t47555 - t47559 + t47560 + t47561 + t47562;
    let t47565 = 256.0 / 243.0 * t40324;
    let t47566 = 64.0 / 15.0 * t40327;
    let t47567 = 64.0 / 45.0 * t40358;
    let t47568 = 32.0 / 15.0 * t40361;
    (t47562, t47563, t47565, t47566, t47567, t47568)
}
