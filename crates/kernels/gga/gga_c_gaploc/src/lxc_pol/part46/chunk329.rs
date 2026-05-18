//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 329/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk329<F: Float>(t1445: F, t2572: F, t2581: F, t701: F, t773: F, t954: F, t1: F, t935: F, t106: F, t316: F, t769: F, t774: F) -> (F, F, F, F, F) {
    let t2642 = t1445 * t2572;
    let t2645 = t2581 * t701;
    let t2646 = t1445 * t2645;
    let t2649 = t773 * t954;
    let t2652 = t935 * t1;
    let t2653 = t2652 * t106;
    let t2654 = t2653 * t316;
    let t2657 = t769 * t774;
    (t2642, t2646, t2649, t2654, t2657)
}
