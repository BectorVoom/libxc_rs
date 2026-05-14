//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 865/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk865<F: Float>(t5359: F, t7601: F, t7603: F, t7605: F, t7607: F, t7609: F, t7613: F, t7615: F, t7617: F, t7619: F, t7621: F, t7623: F, t7625: F, t7629: F, t7634: F, t7636: F, t7637: F) -> (F,) {
    let t8443 = t7601 + t7603 + t7605 + t7607 + t7609 - t7613 + t7615 + t7617 + t7619 + t7621 + t7623 - t7625 + t7629 + t7634 - t7636 - t7637 + t5359;
    (t8443,)
}
