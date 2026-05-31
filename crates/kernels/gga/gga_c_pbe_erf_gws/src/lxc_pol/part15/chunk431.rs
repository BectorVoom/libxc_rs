//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 431/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk431<F: Float>(t617: F, t649: F, t661: F, t1621: F, t1620: F, t586: F, t632: F) -> (F, F, F, F) {
    let t1622 = t649 * t617;
    let t1623 = t1622 * t661;
    let t1624 = t1621 * t1623;
    let t1626 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1620 * t1624;
    let t1627 = t632 * t586;
    (t1623, t1624, t1626, t1627)
}
