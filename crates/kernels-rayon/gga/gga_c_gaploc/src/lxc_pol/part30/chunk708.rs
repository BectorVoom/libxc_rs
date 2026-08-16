//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 708/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk708(t1538: f64, t6578: f64, t6583: f64, t1407: f64, t2361: f64, t203: f64, t883: f64, t900: f64, t4384: f64, t2470: f64, t549: f64, t1416: f64) -> (f64, f64, f64, f64) {
    let t6584 = t1538 * t6578;
    let t6585 = t6583 * t6584;
    let t6587 = t1407 * t2361;
    let t6589 = t883 * t203;
    let t6590 = t900 * t6589;
    let t6591 = t4384 * t6590;
    let t6593 = t549 * t2470;
    let t6594 = t1416 * t6593;
    (t6585, t6587, t6591, t6594)
}
