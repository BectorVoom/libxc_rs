//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1121/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1121<F: Float>(t190: F, t5589: F, t674: F, t8451: F, t11395: F, t5: F, t25708: F, t4055: F, t8452: F, t11261: F, t13337: F, t1416: F, t1672: F, t1: F, t516: F, t619: F, t6803: F, t8379: F) -> (F, F, F, F, F) {
    let t35680 = t8451 * t190 * t674 * t5589;
    let t35682 = t5 * t11395;
    let t35685 = t35682 * t25708 * t8452 * t4055;
    let t35689 = t11261 * t1416 * t1672 * t13337;
    let t35694 = t8379 * t516 * t1 * t6803 * t619;
    (t35680, t35682, t35685, t35689, t35694)
}
