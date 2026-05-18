//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 203/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk203<F: Float>(t418: F, t572: F, t571: F, t11: F, t570: F, t173: F, t184: F) -> (F, F, F, F, F, F) {
    let t573 = t572 * t418;
    let t574 = t571 * t573;
    let t575 = t11 * t574;
    let t577 = t570 + F::new(0.18891666666666666667e-2) * t575;
    let t578 = t173 * t577;
    let t579 = t578 * t184;
    (t573, t574, t575, t577, t578, t579)
}
