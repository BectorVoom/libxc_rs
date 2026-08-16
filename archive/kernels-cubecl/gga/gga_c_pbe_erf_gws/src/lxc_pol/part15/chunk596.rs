//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 596/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk596<F: Float>(t2705: F, t625: F, t2704: F, t1041: F, t401: F, t1714: F, t2679: F, t2673: F, t657: F, t1472: F, t21: F) -> (F, F, F, F, F, F) {
    let t2706 = t625 * t2705;
    let t2707 = t2704 * t2706;
    let t2710 = t401 * t1041;
    let t2712 = t1714 * t2679;
    let t2715 = t657 * t2673;
    let t2718 = t21 * t1472;
    (t2706, t2707, t2710, t2712, t2715, t2718)
}
