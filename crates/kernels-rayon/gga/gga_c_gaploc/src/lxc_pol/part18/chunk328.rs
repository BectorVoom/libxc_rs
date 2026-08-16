//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 328/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk328(t1415: f64, t546: f64, t1353: f64, t549: f64, t1: f64, t1359: f64, t544: f64, t106: f64, t408: f64) -> (f64, f64, f64, f64, f64) {
    let t1416 = t1415 * t546;
    let t1417 = t549 * t1353;
    let t1420 = t1359 * t1;
    let t1421 = t544 * t1420;
    let t1422 = t106 * t408;
    (t1416, t1417, t1420, t1421, t1422)
}
