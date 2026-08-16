//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1110/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1110(t3073: f64, t4241: f64, t6482: f64, t6461: f64, t11545: f64, t11549: f64, t11552: f64, t11557: f64, t11560: f64, t11566: f64, t11570: f64, t11574: f64, t11578: f64, t11582: f64, t11586: f64, t11596: f64, t19394: f64, t19396: f64, t19397: f64, t19398: f64, t19399: f64) -> (f64, f64, f64) {
    let t19894 = t3073 * t6482 * t4241;
    let t19898 = t3073 * t6461 * t4241;
    let t19911 = t11545 + t11549 - t11552 + t19394 - t11557 - t11560 - t19396 - t19397 + t19398 + t11566 + t11570 - t11574 + t11578 - t11582 - t11586 - t19399 + t11596;
    (t19894, t19898, t19911)
}
