//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1177/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1177<F: Float>(t16572: F, t16574: F, t47527: F, t47528: F, t47529: F, t47530: F, t47535: F, t47536: F, t47538: F, t47545: F, t47546: F, t16595: F, t47547: F, t47548: F, t47552: F, t47554: F, t47555: F, t47559: F, t47560: F, t47561: F, t47562: F, t47565: F) -> (F, F) {
    let t48629 = t47527 - t47528 + t47529 + t47530 + t47535 - t47536 - t47538 + t16572 - t47545 + t16574 + t47546;
    let t48630 = t47547 - t47548 + t16595 + t47552 + t47554 - t47555 - t47559 + t47560 + t47561 + t47562 + t47565;
    (t48629, t48630)
}
