//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta271 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1120;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1121;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1122;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1123;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1124;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta271(t1774: f64, t2039: f64, t109: f64, t7053: f64, t7464: f64, t510: f64, t1458: f64, t2075: f64, t2057: f64, t7475: f64, t1492: f64, t2047: f64, t7074: f64, t7076: f64, t7078: f64, t7082: f64, t7494: f64, t7498: f64, t7501: f64, t7504: f64, t7506: f64, t7508: f64, t218: f64, t1527: f64, t2053: f64, t2718: f64, t1510: f64, t7101: f64, t235: f64, t1499: f64, t2051: f64, t226: f64, t7095: f64, t7097: f64, t7522: f64, t7526: f64, t7530: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7796 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1120(t1774, t2039);
        let t7801 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1121(t109, t7053, t7464);
        let t7802 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1122(t510, t7801);
        let (t7806, t7809, t7815, t7823) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1123(t1458, t2075, t2057, t7475, t1492, t2047, t7074, t7076, t7078, t7082, t7494, t7498, t7501, t7504, t7506, t7508);
        let (t7824, t7830) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1124(t218, t7823, t1527, t2053, t2718);
        let (t7837, t7839, t7841) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1125(t1510, t7101, t235, t7823, t1499, t2051, t226, t7095, t7097, t7522, t7526, t7530, t812);
    (t7796, t7801, t7802, t7806, t7809, t7815, t7823, t7824, t7830, t7837, t7839, t7841)
}
