//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta76 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk552;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk553;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk554;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk555;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk556;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk557;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk558;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk559;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta76(t1547: f64, t894: f64, t901: f64, t1539: f64, t908: f64, t136: f64, t1541: f64, t899: f64, t907: f64, t913: f64, t893: f64, t917: f64, t926: f64, t929: f64, t932: f64, t936: f64, t324: f64, t945: f64, t948: f64, t951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1548, t1551, t1553, t1554, t1556) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk552(t1547, t894, t901, t1539, t908, t136, t1541, t899, t907);
        let t1557 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk553(t1556, t913);
        let (t1559, t1561) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk554(t1557, t893, t1541, t917);
        let t1568 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk555(t1541, t1548, t1551, t1554, t926, t929);
        let t1569 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk556(t1568, t932);
        let t1573 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk557(t1541, t936);
        let (t1574, t1580) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk558(t1573, t324, t1541, t1548, t1551, t1554, t945, t948);
        let t1581 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk559(t1580, t951);
    (t1553, t1554, t1556, t1557, t1559, t1561, t1568, t1569, t1573, t1574, t1580, t1581)
}
