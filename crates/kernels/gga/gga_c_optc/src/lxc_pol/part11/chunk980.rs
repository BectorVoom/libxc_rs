//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 980/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk980<F: Float>(t23: F, t3086: F, t191: F, t1574: F, t490: F, t9226: F, t1220: F, t1578: F, t3902: F, t1570: F, t9227: F, t1523: F, t8996: F, t1111: F, t1502: F, t1781: F) -> (F, F, F, F, F, F) {
    let t34028 = t23 * t3086;
    let t34029 = t34028 * t191;
    let t34107 = t490 * t1574 * t9226;
    let t34301 = t1220 * t3902 * t1578;
    let t34309 = t1570 * t9227;
    let t34319 = t1523 * t8996;
    let t34350 = t1111 * t1781 * t1502;
    (t34029, t34107, t34301, t34309, t34319, t34350)
}
