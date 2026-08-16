//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1263/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1263<F: Float>(t1863: F, t96469: F, t2240: F, t5399: F, t27948: F, t33: F, t55921: F, t6489: F, t12571: F, t26083: F, t1862: F, t5392: F) -> (F, F, F, F, F, F) {
    let t96470 = t1863 * t96469;
    let t96473 = t2240 * t5399;
    let t96529 = t2240 * t33 * t27948;
    let t96532 = t55921 * t6489;
    let t96538 = t12571 * t26083;
    let t96547 = t2240 * t5392 * t1862;
    (t96470, t96473, t96529, t96532, t96538, t96547)
}
