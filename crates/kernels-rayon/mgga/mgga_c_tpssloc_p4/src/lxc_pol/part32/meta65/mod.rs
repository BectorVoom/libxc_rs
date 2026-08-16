//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta65 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk429;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk430;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk431;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk432;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk433;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk434;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk435;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta65(t1090: f64, t1230: f64, t248: f64, t1174: f64, t1195: f64, t1198: f64, t1203: f64, t1213: f64, t1218: f64, t1224: f64, t1227: f64, t488: f64, t466: f64, t225: f64, t492: f64, t496: f64, t68: f64, t1011: f64, t1209: f64, t1206: f64, t1215: f64, t491: f64, t357: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1232 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk429(t1090, t1230, t248);
        let t1235 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk430(t1174, t1195, t1198, t1203, t1213, t1218, t1224, t1227, t1232, t488);
        let (t1236, t1238) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk431(t1235, t466, t225, t492);
        let (t1239, t1240) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk432(t496);
        let t1241 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk433(t1240, t68);
        let t1243 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk434(t1011, t1209);
        let t1244 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk435(t1206, t1243);
        let (t1245, t1246) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk436(t1215, t491, t357, t475);
    (t1232, t1235, t1236, t1238, t1239, t1240, t1241, t1243, t1244, t1245, t1246)
}
