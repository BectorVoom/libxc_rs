//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk578;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk579;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk580;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta81(t1129: f64, t1148: f64, t1659: f64, t1673: f64, t1675: f64, t1683: f64, t1688: f64, t1695: f64, t300: f64, t436: f64, t1147: f64, t1156: f64, t1694: f64, t1164: f64, t1420: f64, t338: f64, t1178: f64, t1409: f64, t1177: f64, t1111: f64, t1668: f64, t457: f64, t460: f64, t974: f64, t1173: f64, t1174: f64, t463: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1699, t1701, t1703) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk578(t1129, t1148, t1659, t1673, t1675, t1683, t1688, t1695, t300, t436, t1147, t1156, t1694);
        let (t1705, t1706) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk579(t1164, t1703, t1420, t338);
        let (t1709, t1710, t1714) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk580(t1178, t1409, t1177, t1111, t1668);
        let (t1716, t1720) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk581(t1714, t457, t460, t974, t1173, t1174, t1706, t1710, t463);
    (t1699, t1701, t1703, t1705, t1706, t1709, t1714, t1716, t1720)
}
