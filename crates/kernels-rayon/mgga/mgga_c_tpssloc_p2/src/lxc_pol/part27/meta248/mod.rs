//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1196;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1197;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1198;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1199;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1200;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1201;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1202;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1203;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta248(t1930: f64, t6739: f64, t1934: f64, t344: f64, t1009: f64, t1014: f64, t363: f64, t1022: f64, t360: f64, t68: f64, t1004: f64, t1941: f64, sigma0: f64, t1018: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6740 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1196(t1930, t6739);
        let (t6741, t6742) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1197(t1934, t344, t6740);
        let t6743 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1198(t1009, t1014);
        let t6744 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1199(t363, t6743);
        let t6746 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1200(t1022, t360, t68);
        let t6747 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1201(t6744, t6746);
        let (t6750, t6753) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1202(t1004, t1941, t1014, sigma0);
        let t6754 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1203(t1018, t6753);
        let t6755 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1204(t1012, t6754);
    (t6740, t6741, t6742, t6743, t6744, t6746, t6747, t6750, t6753, t6754, t6755)
}
