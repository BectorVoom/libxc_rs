//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 881/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk881<F: Float>(t2660: F, t2796: F, t2800: F, t1879: F, t1033: F, t1726: F, t1733: F, t209: F, t184: F, t1024: F, t7593: F, t7595: F, t7597: F, t7599: F, t7601: F, t7603: F, t7605: F, t7607: F, t7609: F, t7613: F, t7615: F, t7617: F) -> (F, F, F, F, F, F) {
    let t7619 = F::new(16.0) / F::new(45.0) * t2660 * t2796;
    let t7621 = F::new(8.0) / F::new(15.0) * t2660 * t2800;
    let t7623 = F::new(16.0) / F::new(45.0) * t1879 * t2796;
    let t7625 = F::new(2.0) / F::new(15.0) * t1033 * t1726;
    let t7626 = t1733 * t209;
    let t7627 = t7626 * t184;
    let t7629 = F::new(4.0) / F::new(15.0) * t7627 * t1024;
    let t7630 = t7593 + t7595 + t7597 + t7599 + t7601 + t7603 + t7605 + t7607 + t7609 - t7613 + t7615 + t7617 + t7619 + t7621 + t7623 - t7625 + t7629;
    (t7619, t7621, t7623, t7625, t7629, t7630)
}
