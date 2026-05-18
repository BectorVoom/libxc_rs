//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 864/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk864<F: Float>(t4991: F, t597: F, t1828: F, t587: F, t16595: F, t16597: F, t16599: F, t16601: F, t16603: F, t16605: F, t16609: F, t16611: F, t16616: F, t16620: F) -> (F, F) {
    let t16621 = t4991 * t597;
    let t16623 = t587 * t16621 * t1828;
    let t16624 = F::new(32.0) / F::new(135.0) * t16623;
    let t16625 = t16595 + t16597 + t16599 - t16601 - t16603 + t16605 + t16609 - t16611 + t16616 + t16620 + t16624;
    (t16624, t16625)
}
