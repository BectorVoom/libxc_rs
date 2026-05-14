//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 710/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk710<F: Float>(t1026: F, t8232: F, t1882: F, t3463: F, t3590: F, t379: F, t569: F, t1647: F, t3445: F, t2221: F, t558: F, t574: F, t2142: F, t3565: F, t144: F, t1053: F, t9428: F) -> (F, F, F, F, F, F, F, F) {
    let t12617 = t8232 * t1026;
    let t12620 = 2.0 / 27.0 * t1882 * t3463;
    let t12622 = t569 * t3590 * t379;
    let t12625 = t3445 * t1647;
    let t12626 = t2221 * t12625;
    let t12630 = t574 * t3590 * t558;
    let t12633 = t2142 * t3565;
    let t12634 = t144 * t12633;
    let t12637 = t9428 * t1053;
    (t12617, t12620, t12622, t12626, t12630, t12633, t12634, t12637)
}
