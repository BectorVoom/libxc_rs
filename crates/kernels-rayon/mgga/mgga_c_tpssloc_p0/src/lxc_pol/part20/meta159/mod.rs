//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1008;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1009;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1010;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1011;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1012;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1013;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1014;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta159(t240: f64, t3788: f64, t1336: f64, t1351: f64, t550: f64, t1343: f64, t820: f64, t1339: f64, t835: f64, t1354: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3789, t3790, t3791) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1008(t240, t3788, t1336, t1351);
        let t3792 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1009(t550);
        let t3793 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1010(t3791, t3792);
        let t3795 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1011(t1343, t3793, t820);
        let t3798 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1012(t1339, t835);
        let t3799 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1013(t1336, t3798);
        let (t3800, t3802) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1014(t1354, t3799, t1339, t242);
        let t3803 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1015(t1336, t3802);
    (t3789, t3790, t3791, t3792, t3793, t3795, t3798, t3799, t3800, t3802, t3803)
}
