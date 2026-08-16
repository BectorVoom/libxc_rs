//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta53 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk362;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk363;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk364;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk365;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk366;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk367;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk368;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta53(t1017: f64, t368: f64, t1015: f64, t1012: f64, t376: f64, t61: f64, t890: f64, t916: f64, t956: f64, t958: f64, t963: f64, t360: f64, t248: f64, t34: f64, t365: f64, t35: f64, t364: f64, t354: f64, t122: f64, t374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1018, t1019) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk362(t1017, t368, t1015);
        let t1020 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk363(t1012, t1019);
        let t1021 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk364(t376, t61);
        let t1022 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk365(t890, t916, t956, t958, t963);
        let t1023 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk366(t1022, t360);
        let t1025 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk367(t1021, t1023, t248);
        let t1030 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk368(t34, t365, t35);
        let (t1031, t1032, t1036) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk369(t1030, t364, t354, t122, t374, t376);
    (t1018, t1019, t1020, t1021, t1022, t1023, t1025, t1030, t1031, t1032, t1036)
}
