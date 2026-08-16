//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 477/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk477(t2674: f64, t285: f64, t191: f64, t2254: f64, t332: f64, t330: f64, t197: f64, t617: f64, t936: f64, t1854: f64, t942: f64, t1882: f64, t320: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2675 = t2674 * t285;
    let t2676 = t2675 * t191;
    let t2677 = t332 * t2254;
    let t2678 = t330 * t2677;
    let t2679 = t197 * t2678;
    let t2682 = t617 * t936;
    let t2685 = t1854 * t942;
    let t2690 = t320 * t1882;
    (t2675, t2676, t2679, t2682, t2685, t2690)
}
