//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta62 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk406;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk407;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk408;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk409;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk410;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta62(t1089: f64, t461: f64, t607: f64, t1177: f64, t1111: f64, t1115: f64, t457: f64, t460: f64, t974: f64, t1173: f64, t1174: f64, t491: f64, t1169: f64, t221: f64, t456: f64, t1176: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1178 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk406(t1089, t461);
        let (t1179, t1180, t1184) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk407(t1178, t607, t1177, t1111, t1115);
        let t1186 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk408(t1184, t457, t460);
        let (t1187, t1190) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk409(t1186, t974, t1173, t1174, t1180);
        let (t1191, t1193) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk410(t1190, t491, t1169, t221);
        let (t1195, t1196) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk411(t1193, t456, t1089, t1176);
    (t1178, t1179, t1180, t1184, t1186, t1187, t1190, t1191, t1193, t1195, t1196)
}
