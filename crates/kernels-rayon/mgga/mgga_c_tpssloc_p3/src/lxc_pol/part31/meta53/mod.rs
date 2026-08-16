//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta53 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk346;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk347;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk348;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk349;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk350;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk351;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk352;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta53(t1010: f64, t1011: f64, t361: f64, t363: f64, t336: f64, t371: f64, t368: f64, t376: f64, t61: f64, t890: f64, t916: f64, t956: f64, t958: f64, t963: f64, t360: f64, t248: f64, t34: f64, t365: f64, t35: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1012, t1013, t1014, t1015) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk346(t1010, t1011, t361, t363);
        let t1017 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk347(t336, t371);
        let t1019 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk348(t1017, t368, t1015);
        let t1020 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk349(t1012, t1019);
        let t1021 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk350(t376, t61);
        let t1022 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk351(t890, t916, t956, t958, t963);
        let t1023 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk352(t1022, t360);
        let (t1025, t1030) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk353(t1021, t1023, t248, t34, t365, t35);
    (t1012, t1013, t1014, t1015, t1017, t1019, t1020, t1021, t1022, t1023, t1025, t1030)
}
