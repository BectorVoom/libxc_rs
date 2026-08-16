//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk848;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta190(t2710: f64, t814: f64, t829: f64, t252: f64, t9971: f64, t9976: f64, t2728: f64, t9981: f64, t2684: f64, t2732: f64, t6647: f64, t9632: f64, t2678: f64, t860: f64, t9661: f64, t10016: f64, t10055: f64, t10058: f64, t10069: f64, t10073: f64, t226: f64, t255: f64, t2613: f64, t2617: f64, t2729: f64, t2733: f64, t2736: f64, t2738: f64, t2740: f64, t4281: f64, t4291: f64, t808: f64, t812: f64, t861: f64, t863: f64, t9612: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10076, t10077, t10080, t10081, t10084, t10091, t10094) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk848(t2710, t814, t829, t252, t9971, t9976, t2728, t9981, t2684, t2732, t6647, t9632);
        let (t10097, t10098, t10101, t10103) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk849(t252, t2678, t829, t860, t9661, t10016, t10055, t10058, t10069, t10073, t10077, t10081, t10084, t10091, t10094, t226, t255, t2613, t2617, t2729, t2733, t2736, t2738, t2740, t4281, t4291, t808, t812, t861, t863, t9612);
    (t10076, t10077, t10080, t10081, t10084, t10091, t10094, t10097, t10098, t10101, t10103)
}
