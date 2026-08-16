//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta76 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk503;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk504;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk505;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk506;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk507;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk508;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk509;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta76(t1492: f64, t252: f64, t119: f64, t1484: f64, t210: f64, t225: f64, t237: f64, t1464: f64, t1473: f64, t1476: f64, t680: f64, t705: f64, t752: f64, t760: f64, t765: f64, t824: f64, t228: f64, t230: f64, t232: f64, t819: f64, t820: f64, t847: f64, t249: f64, t787: f64, t803: f64, t817: f64, t840: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1493, t1495, t1496) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk503(t1492, t252, t119, t1484, t210);
        let t1499 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk504(t1492, t225);
        let (t1500, t1504) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk505(t1499, t237, t1464, t1473, t1476, t225, t680, t705, t752, t760, t765);
        let (t1506, t1509) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk506(t1484, t824, t1504, t228, t230);
        let t1510 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk507(t1509, t232);
        let t1512 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk508(t1510, t819, t820);
        let t1516 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk509(t1484, t820, t847);
        let t1519 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk510(t1496, t1500, t1512, t1516, t249, t787, t803, t817, t840, t843);
    (t1493, t1495, t1496, t1499, t1500, t1504, t1506, t1509, t1510, t1512, t1516, t1519)
}
