//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1138;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1139;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta346(t154: f64, t1995: f64, t205: f64, t12247: f64, t551: f64, t236: f64, t1336: f64, t240: f64, t3792: f64, t10021: f64, t1361: f64, t22843: f64, t241: f64, t67: f64, t1339: f64, t2690: f64, t3788: f64, t6924: f64, t246: f64, t39037: f64, t522: f64, t2221: f64, t3824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40025, t40041, t40044, t40046, t40059, t40070) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1138(t154, t1995, t205, t12247, t551, t236, t1336, t240, t3792, t10021, t1361, t22843, t241, t67);
        let (t40123, t40159, t40168, t40224, t40227) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1139(t10021, t1336, t1339, t2690, t3788, t67, t6924, t246, t39037, t522, t2221, t3824);
    (t40025, t40041, t40044, t40046, t40059, t40070, t40123, t40159, t40168, t40224, t40227)
}
