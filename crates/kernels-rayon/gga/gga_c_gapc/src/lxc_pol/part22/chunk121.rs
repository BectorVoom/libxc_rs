//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 121/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk121(t1: f64, t44: f64, t350: f64, t55: f64, t78: f64, t46: f64, t51: f64, t352: f64, t354: f64, t358: f64, t360: f64, t54: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t367 = t44 * t1;
    let t369 = t350 * t78 * t55;
    let t371 = 0.18311555036753159941e-3_f64 * t367 * t369;
    let t372 = t44 * t46;
    let t373 = t51 * t51;
    let t374 = 1.0_f64 / t373;
    let t379 = -0.86308333333333333334e0_f64 * t352 - 0.301925e0_f64 * t354 - 0.5501625e-1_f64 * t358 - 0.82785e-1_f64 * t360;
    let t381 = 1.0_f64 / t54;
    (t367, t369, t371, t372, t373, t374, t379, t381)
}
