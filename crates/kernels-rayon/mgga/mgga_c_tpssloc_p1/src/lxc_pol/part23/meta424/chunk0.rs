//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1252/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1252(t10480: f64, t21391: f64, t248: f64, t3101: f64, t1041: f64, t10457: f64, t21118: f64, t1020: f64, t21595: f64, t14511: f64, t17655: f64, t10883: f64, t21403: f64) -> (f64, f64, f64, f64, f64) {
    let t70227 = t10480 * t248 * t3101 * t21391;
    let t70239 = t1041 * t248 * t10457 * t21118;
    let t70346 = t1020 * t248 * t3101 * t21595;
    let t70351 = t14511 * t17655;
    let t70363 = t10883 * t248 * t3101 * t21403;
    (t70227, t70239, t70346, t70351, t70363)
}
