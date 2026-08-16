//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1687/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1687<F: Float>(t2240: F, t608: F, t1864: F, t645: F, t1863: F, t6489: F, t9231: F, t192: F, t532: F, t1982: F) -> (F, F, F, F, F, F) {
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    let t22551 = t1863 * t22550;
    let t22554 = t9231 * t6489;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    (t22549, t22550, t22551, t22554, t22573, t22574)
}
