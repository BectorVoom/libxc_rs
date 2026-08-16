//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 517/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk517(t1445: f64, t2582: f64, t2089: f64, t935: f64, t723: f64, t1: f64, t2536: f64) -> (f64, f64, f64, f64, f64) {
    let t2664 = t1445 * t2582;
    let t2667 = t2089 * t935;
    let t2668 = t2667 * t723;
    let t2669 = t1445 * t2668;
    let t2672 = t2536 * t1;
    (t2664, t2667, t2668, t2669, t2672)
}
