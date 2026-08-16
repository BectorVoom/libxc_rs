//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta115 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk691;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk692;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk693;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk694;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk695;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta115(t2717: f64, t68: f64, t252: f64, t2627: f64, t814: f64, t852: f64, t261: f64, t1878: f64, t268: f64, t271: f64, t690: f64, t885: f64, t1043: f64, t154: f64, t632: f64, t2289: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2718 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk691(t2717, t68);
        let (t2728, t2732, t2751, t2752) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk692(t252, t2627, t814, t852, t261);
        let (t2764, t2765, t2766) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk693(t1878, t268, t271, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk694(t1043, t154);
        let (t2769, t2770) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk695(t632);
        let t2775 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk696(t2289);
    (t2718, t2728, t2732, t2751, t2752, t2764, t2765, t2766, t2768, t2769, t2770, t2775)
}
