//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1410/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1410<F: Float>(t2403: F, t3298: F, t11220: F, t699: F, t1114: F, t9709: F, t3304: F, t3301: F, t1102: F, t11258: F, t3270: F, t3287: F) -> (F, F, F, F, F, F, F) {
    let t43855 = t2403 * t3298;
    let t43857 = t699 * t11220;
    let t43859 = t9709 * t1114;
    let t43861 = t2403 * t3304;
    let t43863 = t2403 * t3301;
    let t43866 = t3270 * t11258 * t1102;
    let t43869 = t3287 * t11258 * t1102;
    (t43855, t43857, t43859, t43861, t43863, t43866, t43869)
}
