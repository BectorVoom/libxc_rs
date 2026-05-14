//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1097/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1097<F: Float>(t683: F, t9942: F, t1434: F, t2399: F, t6124: F, t42050: F, t91: F, t2404: F, t2506: F, t2347: F, t6061: F, t2360: F, t6109: F, t6111: F, t1636: F, t6144: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t97078 = t683 * t9942;
    let t97123 = t1434 * t2399 * t6124;
    let t97168 = t91 * t42050;
    let t97181 = t2404 * t2506;
    let t97190 = t6061 * t2347;
    let t97198 = t6061 * t2360;
    let t97232 = t6109 * t2399 * t6111;
    let t97244 = t89 * t1636 * t6144;
    (t97078, t97123, t97168, t97181, t97190, t97198, t97232, t97244)
}
