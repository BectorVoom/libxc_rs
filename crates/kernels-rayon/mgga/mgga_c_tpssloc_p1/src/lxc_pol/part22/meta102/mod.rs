//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta102 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk695;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk696;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk697;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk698;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk699;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk700;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk701;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta102(t1878: f64, t268: f64, t271: f64, t690: f64, t885: f64, t1043: f64, t154: f64, t632: f64, t2289: f64, t888: f64, t892: f64, t287: f64, t891: f64, t275: f64, t273: f64, t276: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2764, t2765, t2766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk695(t1878, t268, t271, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk696(t1043, t154);
        let t2769 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk697(t632);
        let t2770 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk698(t2769);
        let t2775 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk699(t2289);
        let (t2787, t2790, t2791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk700(t888, t892, t287, t891);
        let t2792 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk701(t275, t2791);
        let t2798 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk702(t273, t276);
    (t2764, t2765, t2766, t2768, t2769, t2770, t2775, t2787, t2790, t2791, t2792, t2798)
}
