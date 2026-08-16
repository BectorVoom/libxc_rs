//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta24 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk171;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk172;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk173;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk174;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk175;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk176;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk177;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk178;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk179;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk180;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta24(t405: f64, t60: f64, t417: f64, t221: f64, t456: f64, t225: f64, t68: f64, t358: f64, t425: f64, t453: f64, t455: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t457 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk171(t405);
        let (t458, t460) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk172(t457, t60, t417);
        let t461 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk173(t460);
        let t462 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk174(t458, t461);
        let (t463, t466) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk175(t221, t462, t456);
        let t467 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk176(t221, t458);
        let t470 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk177(t225, t466);
        let t471 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk178(t470, t68);
        let t475 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk179(t225, t358, t425, t453, t455);
        let (t476, t477) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk180(t475);
    (t457, t460, t461, t462, t463, t466, t467, t470, t471, t475, t476, t477)
}
