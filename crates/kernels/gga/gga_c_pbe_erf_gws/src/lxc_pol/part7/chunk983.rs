//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 983/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk983<F: Float>(t16609: F, t16611: F, t16616: F, t16620: F, t16624: F, t16630: F, t16633: F, t16636: F, t16639: F, t16642: F, t16645: F, t5202: F, t735: F) -> (F, F) {
    let t18159 = t16609 - t16611 + t16616 + t16620 + t16624 + t16630 - t16633 - t16636 - t16639 + t16642 + t16645;
    let t18160 = t5202 * t735;
    (t18159, t18160)
}
