//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 957/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk957(t20865: f64, t8392: f64, t20869: f64, t1882: f64, t20942: f64, t20935: f64, t20698: f64, t20755: f64, t20899: f64, t20720: f64, t1526: f64, t20514: f64, t7705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78438 = t8392 * t20865;
    let t78565 = t8392 * t20869;
    let t78573 = t1882 * t20942;
    let t78584 = t8392 * t20935;
    let t78601 = t1882 * t20698;
    let t78603 = t8392 * t20755;
    let t78605 = t1882 * t20899;
    let t78618 = t1882 * t20720;
    let t78650 = t1526 * t7705 * t20514;
    (t78438, t78565, t78573, t78584, t78601, t78603, t78605, t78618, t78650)
}
