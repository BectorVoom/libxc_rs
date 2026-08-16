//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1225/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1225<F: Float>(t20514: F, t20549: F, t20553: F, t20557: F, t20564: F, t20566: F, t20569: F, t20582: F, t20584: F, t20588: F, t20593: F, t20601: F, t20606: F, t20615: F, t20623: F, t20631: F, t20638: F, t20653: F, t20658: F, t20669: F, t20691: F, t20700: F) -> (F, F) {
    let t21693 = t20514 + t20549 - t20553 - t20557 - t20564 - t20566 + t20569 - t20582 + t20584 - t20588 - t20593;
    let t21694 = t20601 - t20606 + t20615 - t20623 + t20631 + t20638 - t20653 - t20658 + t20669 - t20691 - t20700;
    (t21693, t21694)
}
