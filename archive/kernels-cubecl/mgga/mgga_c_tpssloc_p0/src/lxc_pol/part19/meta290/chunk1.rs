//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1062/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1062<F: Float>(t1307: F, t3792: F, t12368: F, t3805: F, t1328: F, t210: F, t3719: F, t12178: F, t1343: F, t820: F, t3788: F, t835: F) -> (F, F, F, F, F) {
    let t12369 = t3792 * t1307;
    let t12371 = t3805 * t12368 * t12369;
    let t12375 = t210 * t1328 * t3719;
    let t12379 = t1343 * t820 * t12178;
    let t12384 = t3788 * t835;
    (t12369, t12371, t12375, t12379, t12384)
}
