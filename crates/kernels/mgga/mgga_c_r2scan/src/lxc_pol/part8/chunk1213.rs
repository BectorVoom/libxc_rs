//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1213/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1213<F: Float>(t20237: F, t25503: F, t6211: F, t24581: F, t2559: F, t6407: F, t7949: F, t2133: F, t2574: F, t6848: F, t2139: F, t2578: F, t20720: F, t7460: F, t2568: F, t6322: F, t980: F) -> (F, F, F, F, F, F, F, F) {
    let t25505 = t20237 * t6211 * t25503;
    let t25520 = t24581 * t2559;
    let t25521 = 0.64025200389650807209e0 * t25520;
    let t25581 = t6407 * t7949;
    let t25582 = 0.17563392970889009434e0 * t25581;
    let t25584 = t2133 * t6848 * t2574;
    let t25585 = 0.12713391885412927226e1 * t25584;
    let t25604 = t2139 * t6848 * t2578;
    let t25605 = 0.38140175656238781678e1 * t25604;
    let t25606 = t20720 * t7460;
    let t25632 = t2139 * t6848 * t2568;
    let t25633 = 0.38140175656238781678e1 * t25632;
    let t25662 = t980 * t6322;
    (t25505, t25521, t25582, t25585, t25605, t25606, t25633, t25662)
}
