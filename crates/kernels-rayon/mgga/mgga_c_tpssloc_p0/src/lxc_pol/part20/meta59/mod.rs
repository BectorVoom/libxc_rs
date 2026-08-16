//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta59 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk428;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk429;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk430;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk431;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk432;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta59(t1096: f64, t1121: f64, t1124: f64, t1129: f64, t1138: f64, t1144: f64, t1148: f64, t1157: f64, t300: f64, t436: f64, t440: f64, t1147: f64, t1155: f64, t1156: f64, t134: f64, t457: f64, t461: f64, t221: f64, t456: f64, t51: f64, t972: f64, t404: f64, t405: f64, t974: f64, t1089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1161, t1163, t1164) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk428(t1096, t1121, t1124, t1129, t1138, t1144, t1148, t1157, t300, t436, t440);
        let t1166 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk429(t1147, t1155, t1156);
        let (t1168, t1169, t1171, t1173, t1174) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk430(t1164, t1166, t134, t457, t461, t221, t456, t51, t972);
        let t1176 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk431(t404, t405);
        let t1177 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk432(t1176, t974);
        let t1178 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk433(t1089, t461);
    (t1161, t1163, t1164, t1166, t1168, t1169, t1171, t1173, t1174, t1176, t1177, t1178)
}
