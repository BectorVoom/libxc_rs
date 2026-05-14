//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 701/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk701<F: Float>(t3404: F, t549: F, t2030: F, t3383: F, t2057: F, t1014: F, t1995: F, t51: F, t538: F, t6: F, t398: F, t527: F, t1013: F, t2058: F, t542: F, t1008: F, t550: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12418 = t549 * t3404;
    let t12422 = t3383 * t2030;
    let t12425 = t2057 * t3404;
    let t12435 = t1995 * t1014;
    let t12437 = t538 * t6 * t51;
    let t12438 = t12437 * t398;
    let t12441 = t527 * t1014;
    let t12444 = t2058 * t1013;
    let t12445 = t542 * t12444;
    let t12448 = t550 * t1008;
    (t12418, t12422, t12425, t12435, t12438, t12441, t12444, t12445, t12448)
}
