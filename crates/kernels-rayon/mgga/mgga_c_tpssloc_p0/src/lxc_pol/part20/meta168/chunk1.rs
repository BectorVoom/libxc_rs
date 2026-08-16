//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1056/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1056(t1411: f64, t1427: f64, t1434: f64, t3962: f64, t3968: f64, t3971: f64, t3976: f64, t3998: f64, t4018: f64, t609: f64, t629: f64, t642: f64, t66: f64, t80: f64) -> f64 {
    let t4021 = -t3962 * t80 / 12.0_f64 - t3968 * t80 / 12.0_f64 - t3971 * t80 / 12.0_f64 - t1411 * t642 / 12.0_f64 - t3976 * t80 / 12.0_f64 + t3998 * t80 / 24.0_f64 + t1427 * t642 / 24.0_f64 - t609 * t1434 / 12.0_f64 + t629 * t1434 / 24.0_f64 + t66 * t4018 / 24.0_f64;
    t4021
}
