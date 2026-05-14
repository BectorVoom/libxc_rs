//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 728/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk728<F: Float>(t2264: F, t7433: F, t1988: F, t2310: F, t1426: F, t2297: F, t429: F, t598: F, t1298: F, t137: F) -> (F, F, F, F, F) {
    let t8531 = t7433 * t2264;
    let t8533 = t1988 * t2310;
    let t8536 = t1426 * t429 * t2297;
    let t8537 = t598 * t8536;
    let t8539 = t137 * t1298;
    (t8531, t8533, t8536, t8537, t8539)
}
