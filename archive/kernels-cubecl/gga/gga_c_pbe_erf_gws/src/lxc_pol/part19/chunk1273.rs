//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1273/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1273<F: Float>(t13793: F, t56112: F, t3723: F, t859: F, t13792: F, t3738: F, t8599: F, t11660: F, t331: F, t3802: F, t6472: F, t833: F) -> (F, F, F, F) {
    let t56113 = t56112 * t13793;
    let t56115 = t859 * t3723;
    let t56116 = t13792 * t56115;
    let t56118 = t8599 * t3738;
    let t56119 = t13792 * t56118;
    let t56124 = t11660 * t6472 * t3802 * t331 * t833;
    (t56113, t56116, t56119, t56124)
}
