//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1272/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1272<F: Float>(t14617: F, t53571: F, t3912: F, t51580: F, t13793: F, t3723: F, t859: F, t13792: F, t3738: F, t8599: F, t11660: F, t331: F, t3802: F, t6472: F, t833: F) -> (F, F, F, F, F, F) {
    let t56110 = t53571 * t14617;
    let t56112 = t3912 * t51580;
    let t56113 = t56112 * t13793;
    let t56115 = t859 * t3723;
    let t56116 = t13792 * t56115;
    let t56118 = t8599 * t3738;
    let t56119 = t13792 * t56118;
    let t56124 = t11660 * t6472 * t3802 * t331 * t833;
    (t56110, t56112, t56113, t56116, t56119, t56124)
}
