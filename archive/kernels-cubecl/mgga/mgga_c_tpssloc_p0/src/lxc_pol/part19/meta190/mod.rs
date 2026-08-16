//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk848;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta190<F: Float>(t2710: F, t814: F, t829: F, t252: F, t9971: F, t9976: F, t2728: F, t9981: F, t2684: F, t2732: F, t6647: F, t9632: F, t2678: F, t860: F, t9661: F, t10016: F, t10055: F, t10058: F, t10069: F, t10073: F, t226: F, t255: F, t2613: F, t2617: F, t2729: F, t2733: F, t2736: F, t2738: F, t2740: F, t4281: F, t4291: F, t808: F, t812: F, t861: F, t863: F, t9612: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10076, t10077, t10080, t10081, t10084, t10091, t10094) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk848::<F>(t2710, t814, t829, t252, t9971, t9976, t2728, t9981, t2684, t2732, t6647, t9632);
        let (t10097, t10098, t10101, t10103) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk849::<F>(t252, t2678, t829, t860, t9661, t10016, t10055, t10058, t10069, t10073, t10077, t10081, t10084, t10091, t10094, t226, t255, t2613, t2617, t2729, t2733, t2736, t2738, t2740, t4281, t4291, t808, t812, t861, t863, t9612);
    (t10076, t10077, t10080, t10081, t10084, t10091, t10094, t10097, t10098, t10101, t10103)
}
