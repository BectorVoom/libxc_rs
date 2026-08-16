//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2219/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219(t12620: f64, t12633: f64, t12636: f64, t12708: f64, t1410: f64, t1434: f64, t2250: f64, t2255: f64, t2283: f64, t2304: f64, t3961: f64, t3967: f64, t3976: f64, t4018: f64, t608: f64, t609: f64, t642: f64, t7445: f64, t80: f64, t9247: f64, t9260: f64, t9268: f64, t9312: f64) -> f64 {
    let t46080 = -t12633 * t642 / 4.0_f64 - t3967 * t2283 * t80 / 4.0_f64 - t1410 * t9312 * t80 / 12.0_f64 - t3976 * t2304 / 4.0_f64 - t9247 * t7445 * t2250 / 4.0_f64 - t9260 * t1434 / 12.0_f64 - t9268 * t1434 / 4.0_f64 - t2255 * t4018 / 2.0_f64 - t609 * t12620 / 4.0_f64 - t3961 * t2283 * t80 / 4.0_f64 - t608 * t12708 * t80 / 4.0_f64 - t12636 * t642 / 2.0_f64;
    t46080
}
