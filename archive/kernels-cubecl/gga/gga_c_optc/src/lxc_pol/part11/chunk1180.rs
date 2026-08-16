//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1180/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1180<F: Float>(t1220: F, t17442: F, t2367: F, t1217: F, t1218: F, t17926: F, t18178: F, t2911: F, t1199: F, t17574: F, t1213: F, t18204: F, t490: F) -> (F, F, F, F, F) {
    let t53494 = t1220 * t2367 * t17442;
    let t53498 = t1217 * t1218 * t17926;
    let t53510 = t18178 * t2911;
    let t53612 = t17574 * t1199;
    let t53769 = t490 * t18204 * t1213;
    (t53494, t53498, t53510, t53612, t53769)
}
