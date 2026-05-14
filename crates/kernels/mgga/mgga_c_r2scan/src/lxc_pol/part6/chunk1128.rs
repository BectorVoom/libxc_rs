//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1128/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1128<F: Float>(t20450: F, t2232: F, t6068: F, t1234: F, t494: F, t113: F, t2155: F, t6063: F, t2252: F, t6085: F, t6086: F, t2080: F, t2106: F, t6093: F, t6412: F, t6072: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20452 = t20450 * t6068 * t2232;
    let t20454 = t1234 * t494;
    let t20455 = t20454 * t113;
    let t20457 = t2155 * t6063 * t20455;
    let t20459 = t2252 * t494;
    let t20460 = t20459 * t113;
    let t20462 = t6085 * t6086 * t20460;
    let t20464 = t2080 * t2106;
    let t20468 = t6093 * t6086 * t20455;
    let t20470 = t6412 * t6068;
    let t20471 = t20470 * t6072;
    (t20452, t20454, t20455, t20457, t20459, t20462, t20464, t20468, t20470, t20471)
}
