//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 730/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk730<F: Float>(t598: F, t8566: F, t355: F, t506: F, t1083: F, t7458: F, t1980: F, t527: F, t7712: F, t1413: F, t2118: F, t1988: F, t2290: F, t2268: F, t7433: F, t2264: F, t7839: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8567 = t598 * t8566;
    let t8569 = t355 * t506;
    let t8571 = t7458 * t1083 * t8569;
    let t8572 = t1980 * t8571;
    let t8574 = t7712 * t527;
    let t8576 = t2118 * t1413;
    let t8578 = t1988 * t2290;
    let t8580 = t7433 * t2268;
    let t8582 = t7839 * t2264;
    (t8567, t8569, t8571, t8572, t8574, t8576, t8578, t8580, t8582)
}
