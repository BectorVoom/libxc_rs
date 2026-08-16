//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1457;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1458;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1459;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1460;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta388(t3502: f64, t42341: f64, t44696: f64, t23508: f64, t3508: f64, t225: f64, t44657: f64, t1209: f64, t475: f64, t43670: f64, t43672: f64, t43674: f64, t43678: f64, t43683: f64, t43685: f64, t43687: f64, t43695: f64, t43702: f64, t43915: f64, t43924: f64, t43953: f64, t43956: f64, t43958: f64, t43961: f64, t43963: f64, t43966: f64, t43973: f64, t43975: f64, t43979: f64, t43982: f64, t43987: f64, t43989: f64, t43994: f64, t43997: f64, t44000: f64, t44002: f64, t44006: f64, t44072: f64, t44080: f64, t44082: f64, t44085: f64, t44089: f64, t44092: f64, t44369: f64, t44161: f64, t44164: f64, t44167: f64, t44358: f64, t44372: f64, t44375: f64, t44377: f64, t44384: f64, t44388: f64, t44392: f64, t44396: f64, t44400: f64, t1174: f64, t11765: f64, t135: f64, t43763: f64, t44620: f64, t3551: f64, t698: f64, t11545: f64, t43791: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44753, t44754, t44774, t44785, t44786, t44792) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1457(t3502, t42341, t44696, t23508, t3508, t225, t44657, t1209, t475, t43670, t43672, t43674, t43678, t43683, t43685, t43687, t43695, t43702, t43915, t43924);
        let t44793 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1458(t43953, t43956, t43958, t43961, t43963, t43966, t43973, t43975, t43979, t43982, t43987, t43989);
        let t44795 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1459(t43994, t43997, t44000, t44002, t44006, t44072, t44080, t44082, t44085, t44089, t44092, t44369);
        let t44796 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1460(t44161, t44164, t44167, t44358, t44372, t44375, t44377, t44384, t44388, t44392, t44396, t44400);
        let (t44798, t44803, t44805, t44811, t44817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1461(t44792, t44793, t44795, t44796, t1174, t11765, t135, t43763, t44620, t3551, t698, t11545, t43791);
    (t44753, t44754, t44774, t44785, t44786, t44798, t44803, t44805, t44811, t44817)
}
