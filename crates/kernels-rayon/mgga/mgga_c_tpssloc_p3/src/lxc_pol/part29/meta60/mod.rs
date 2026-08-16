//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta60 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk406;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk407;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk408;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk409;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk410;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta60(t1096: f64, t1121: f64, t1124: f64, t1129: f64, t1138: f64, t1144: f64, t1148: f64, t1157: f64, t300: f64, t436: f64, t440: f64, t1147: f64, t1155: f64, t1156: f64, t134: f64, t457: f64, t461: f64, t221: f64, t456: f64, t51: f64, t972: f64, t404: f64, t405: f64, t974: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1161, t1163, t1164) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk406(t1096, t1121, t1124, t1129, t1138, t1144, t1148, t1157, t300, t436, t440);
        let (t1166, t1168, t1169, t1170) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk407(t1147, t1155, t1156, t1164, t134, t457, t461);
        let (t1171, t1173, t1174) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk408(t1170, t221, t456, t51, t972);
        let t1176 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk409(t404, t405);
        let t1177 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk410(t1176, t974);
    (t1161, t1163, t1164, t1166, t1168, t1169, t1170, t1171, t1173, t1174, t1176, t1177)
}
