//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1363;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1364;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1365;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1366;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1367;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta247(t756: f64, t9874: f64, t9727: f64, t9780: f64, t9789: f64, t9793: f64, t9797: f64, t9863: f64, t9865: f64, t9867: f64, t9870: f64, t9872: f64, t118: f64, t753: f64, t2375: f64, t2371: f64, t677: f64, t2374: f64, t2535: f64, t2528: f64, t2509: f64, t745: f64, t9843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9876, t9877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1363(t756, t9874, t9727, t9780, t9789, t9793, t9797, t9863, t9865, t9867, t9870, t9872);
        let t9879 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1364(t118, t753);
        let (t9880, t9881, t9882) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1365(t2375, t9879, t2371, t677);
        let (t9884, t9885) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1366(t2374, t9882, t2535, t677);
        let (t9887, t9888) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1367(t2374, t9885, t2528, t677);
        let (t9890, t9892) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1368(t2374, t9888, t2509, t745, t9843);
    (t9876, t9877, t9879, t9880, t9881, t9882, t9884, t9885, t9887, t9888, t9890, t9892)
}
