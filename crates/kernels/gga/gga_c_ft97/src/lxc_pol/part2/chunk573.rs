//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 573/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk573<F: Float>(t1560: F, t7780: F, t89: F, t1642: F, t369: F, t1556: F, t21: F, t1546: F, t1572: F, t1566: F, t1882: F, t1586: F, t378: F, t1602: F, t66: F, t1616: F) -> (F, F, F, F, F, F, F) {
    let t7782 = t89 * t7780 * t1560;
    let t7793 = t1642 * t369;
    let t7800 = 1.0 / t1556 / t21;
    let t7820 = t89 * t1546 * t1572;
    let t7822 = t1882 * t1566;
    let t7824 = t378 * t1586;
    let t7837 = t1602 * t66;
    let t7838 = t7837 * t1616;
    (t7782, t7793, t7800, t7820, t7822, t7824, t7838)
}
