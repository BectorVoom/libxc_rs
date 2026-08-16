//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1142;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1143;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1144;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1145;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1146;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta238(t510: f64, t6534: f64, t652: f64, t1976: f64, t671: f64, t25: f64, t776: f64, t154: f64, t781: f64, t1879: f64, t1883: f64, t131: f64, t209: f64, t229: f64, t1878: f64, t214: f64, t252: f64, t225: f64, t258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6535 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1141(t510, t6534);
        let (t6537, t6539, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1142(t652, t6535, t1976, t671, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1143(t1879, t6546);
        let (t6549, t6551, t6552) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1144(t1883, t6547, t131, t209, t229, t1878);
        let t6553 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1145(t214, t252);
        let t6554 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1146(t225, t258);
        let t6555 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1147(t6554, t776);
    (t6535, t6537, t6539, t6542, t6546, t6547, t6549, t6551, t6552, t6553, t6554, t6555)
}
