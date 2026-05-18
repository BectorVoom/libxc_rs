//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1230/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1230<F: Float>(t21596: F, t21600: F, t21608: F, t21614: F, t21616: F, t21627: F, t21632: F, t21635: F, t21640: F, t21651: F, t21653: F, t2418: F, t353: F, t814: F, t859: F) -> (F, F) {
    let t21712 = t21596 - t21600 - t21608 + t21614 - t21616 + t21627 + t21632 - t21635 + t21640 - t21651 - t21653;
    let t21724 = t859 * t353 * t2418 * t814;
    (t21712, t21724)
}
