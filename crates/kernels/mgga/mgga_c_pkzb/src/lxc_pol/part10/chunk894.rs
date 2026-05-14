//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 894/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk894<F: Float>(t448: F, t452: F, t1450: F, t987: F, t2507: F, t459: F, t1466: F, t1425: F, t4794: F, t973: F, t1424: F, t7: F, t1430: F, t440: F, t1431: F, t2489: F) -> (F, F, F, F, F, F, F, F) {
    let t6634 = t448 * t452;
    let t6639 = t987 * t1450;
    let t6642 = t2507 * t459;
    let t6645 = t987 * t1466;
    let t6655 = t4794 * t973 * t1425;
    let t6658 = t7 * t1424;
    let t6659 = t1430 * t440;
    let t6662 = t2489 * t1431;
    (t6634, t6639, t6642, t6645, t6655, t6658, t6659, t6662)
}
