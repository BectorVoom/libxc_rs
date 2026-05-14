//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 507/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk507<F: Float>(t1557: F, t160: F, t3188: F, t3439: F, t1017: F, t379: F, t2221: F, t558: F, t167: F, t2185: F, t609: F, t574: F, t605: F, t1026: F, t1882: F, t1060: F, t569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3440 = t160 * t1557;
    let t3441 = t3440 * t3188;
    let t3442 = t3439 * t3441;
    let t3445 = t160 * t1017;
    let t3446 = t3445 * t379;
    let t3447 = t2221 * t3446;
    let t3450 = t1017 * t558;
    let t3452 = t2185 * t167 * t3450;
    let t3455 = t1017 * t609;
    let t3457 = t574 * t605 * t3455;
    let t3460 = t1882 * t1026;
    let t3463 = t569 * t1060 * t379;
    (t3440, t3441, t3442, t3445, t3446, t3447, t3450, t3452, t3455, t3457, t3460, t3463)
}
