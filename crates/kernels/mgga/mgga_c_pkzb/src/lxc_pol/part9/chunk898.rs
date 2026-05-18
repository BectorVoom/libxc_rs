//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 898/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk898<F: Float>(t1430: F, t440: F, t1431: F, t2489: F, t1429: F, t15: F, t2493: F, t82: F, t1436: F, t4810: F, t983: F, t1435: F, t23: F) -> (F, F, F, F, F, F) {
    let t6659 = t1430 * t440;
    let t6662 = t2489 * t1431;
    let t6665 = t15 * t1429;
    let t6668 = t2493 * t82;
    let t6676 = t4810 * t983 * t1436;
    let t6679 = t23 * t1435;
    (t6659, t6662, t6665, t6668, t6676, t6679)
}
