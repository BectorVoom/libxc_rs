//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1007/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1007(t1790: f64, t1792: f64, t533: f64, t6446: f64, t1758: f64, t1784: f64, t6452: f64, t6454: f64, t209: f64, t60: f64, t6472: f64, t6519: f64) -> (f64, f64, f64, f64) {
    let t22281 = 0.64327297288604419288e2_f64 * t1790 * t6446 * t1792 * t533;
    let t22285 = 0.3103500882342370105e4_f64 * t6452 * t1758 * t6454 * t1784;
    let t22287 = t60 * t209;
    let t22290 = 0.13012297059337829057e0_f64 * t22287 * t6519 * t6472;
    (t22281, t22285, t22287, t22290)
}
