//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1110/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1110<F: Float>(t3073: F, t4241: F, t6482: F, t6461: F, t11545: F, t11549: F, t11552: F, t11557: F, t11560: F, t11566: F, t11570: F, t11574: F, t11578: F, t11582: F, t11586: F, t11596: F, t19394: F, t19396: F, t19397: F, t19398: F, t19399: F) -> (F, F, F) {
    let t19894 = t3073 * t6482 * t4241;
    let t19898 = t3073 * t6461 * t4241;
    let t19911 = t11545 + t11549 - t11552 + t19394 - t11557 - t11560 - t19396 - t19397 + t19398 + t11566 + t11570 - t11574 + t11578 - t11582 - t11586 - t19399 + t11596;
    (t19894, t19898, t19911)
}
