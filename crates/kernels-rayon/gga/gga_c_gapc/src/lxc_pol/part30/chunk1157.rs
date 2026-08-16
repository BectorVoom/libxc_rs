//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1157/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1157(t11465: f64, t21084: f64, t612: f64, t11549: f64, t8751: f64, t11399: f64, t5700: f64, t1678: f64, t11397: f64, t632: f64, t11533: f64, t424: f64) -> (f64, f64, f64, f64, f64) {
    let t34328 = t21084 * t612 * t11465;
    let t34330 = t11549 * t8751;
    let t34333 = t11399 * t5700;
    let t34334 = t34333 * t1678;
    let t34335 = t632 * t11397 * t34334;
    let t34337 = t424 * t11533;
    (t34328, t34330, t34333, t34335, t34337)
}
