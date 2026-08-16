//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk849;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk850;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk851;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta126(t1354: f64, t3799: f64, t1339: f64, t242: f64, t1336: f64, t1365: f64, t67: f64, t246: f64, t1307: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t3800, t3802) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk849(t1354, t3799, t1339, t242);
        let t3803 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk850(t1336, t3802);
        let (t3804, t3805) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk851(t1365, t67, t246);
        let t3807 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk852(t1307, t550);
    (t3800, t3802, t3803, t3804, t3805, t3807)
}
