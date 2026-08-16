//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 333/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk333<F: Float>(t1445: F, t2582: F, t2089: F, t935: F, t723: F, t1: F, t2536: F, t787: F, t2576: F, t549: F, t161: F, t1968: F) -> (F, F, F, F, F, F) {
    let t2664 = t1445 * t2582;
    let t2667 = t2089 * t935;
    let t2668 = t2667 * t723;
    let t2669 = t1445 * t2668;
    let t2672 = t2536 * t1;
    let t2673 = t787 * t2672;
    let t2676 = t549 * t2576;
    let t2679 = t161 * t1968;
    (t2664, t2669, t2672, t2673, t2676, t2679)
}
