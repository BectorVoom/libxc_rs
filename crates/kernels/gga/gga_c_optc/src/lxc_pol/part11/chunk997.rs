//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 997/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk997<F: Float>(t2300: F, t4831: F, t2415: F, t4895: F, t5064: F, t7274: F, t999: F, t2352: F, t4851: F, t1325: F, t24442: F, t24513: F, t4941: F, t2668: F, t2674: F, t2595: F) -> (F, F, F, F, F, F, F, F) {
    let t39623 = t4831 * t2300;
    let t40005 = t4895 * t2415;
    let t40120 = t999 * t7274 * t5064;
    let t40188 = t4851 * t2352;
    let t40308 = t24442 * t1325;
    let t40326 = t24513 * t4941;
    let t40328 = t2668 * t40326 * t2674;
    let t40355 = t2595 * t4941;
    (t39623, t40005, t40120, t40188, t40308, t40326, t40328, t40355)
}
