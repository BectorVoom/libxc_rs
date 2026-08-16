//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 331/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk331(t1445: f64, t2582: f64, t2089: f64, t935: f64, t723: f64, t1: f64, t2536: f64, t787: f64, t2576: f64, t549: f64, t161: f64, t1968: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2664 = t1445 * t2582;
    let t2667 = t2089 * t935;
    let t2668 = t2667 * t723;
    let t2669 = t1445 * t2668;
    let t2672 = t2536 * t1;
    let t2673 = t787 * t2672;
    let t2676 = t549 * t2576;
    let t2679 = t161 * t1968;
    (t2664, t2667, t2669, t2672, t2673, t2676, t2679)
}
