//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2108/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108(t1863: f64, t96469: f64, t2240: f64, t5399: f64, t22544: f64, t22549: f64, t22551: f64, t26009: f64, t26013: f64, t26016: f64, t90114: f64, t90192: f64, t90248: f64, t90251: f64, t90330: f64, t96443: f64, t96454: f64, t96458: f64, t96462: f64, t96466: f64) -> f64 {
    let t96470 = t1863 * t96469;
    let t96473 = t2240 * t5399;
    let t96478 = -10.0_f64 / 3.0_f64 * t96443 * t22551 - 10.0_f64 / 3.0_f64 * t26016 * t90248 - 10.0_f64 * t90330 * t26009 - 10.0_f64 / 3.0_f64 * t90114 * t26013 - 10.0_f64 * t90192 * t26009 - 10.0_f64 / 3.0_f64 * t22549 * t96454 - 10.0_f64 * t22544 * t96458 - 10.0_f64 / 3.0_f64 * t22549 * t96462 - 5.0_f64 * t22544 * t96466 - 5.0_f64 / 3.0_f64 * t22549 * t96470 - 5.0_f64 / 3.0_f64 * t96473 * t22551 - 10.0_f64 / 3.0_f64 * t26016 * t90251;
    t96478
}
