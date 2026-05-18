//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 792/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk792<F: Float>(t1013: F, t2058: F, t542: F, t1008: F, t550: F, t1995: F, t527: F, t133: F, t1595: F, t929: F, t120: F, t378: F) -> (F, F, F, F, F) {
    let t12444 = t2058 * t1013;
    let t12445 = t542 * t12444;
    let t12448 = t550 * t1008;
    let t12449 = t1995 * t12448;
    let t12452 = t527 * t12448;
    let t12455 = t133 * t12444;
    let t12462 = t929 * t1595;
    let t12464 = t378 * t12462 * t120;
    (t12445, t12449, t12452, t12455, t12464)
}
