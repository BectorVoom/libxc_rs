//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta247 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1157;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1158;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1159;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1160;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1161;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1162;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta247(t252: f64, t828: f64, t232: f64, t6646: f64, t1888: f64, t1894: f64, t852: f64, t214: f64, t1880: f64, t1902: f64, t814: f64, t829: f64, t235: f64, t6624: f64, t1909: f64, t226: f64, t6636: f64, t6641: f64, t6645: f64, t808: f64, t812: f64, t858: f64, t1912: f64, t259: f64, t2597: f64, t2713: f64, t6549: f64, t6557: f64, t6565: f64, t6569: f64, t6574: f64, t6576: f64, t6625: f64, t6627: f64, t6632: f64, t855: f64, t866: f64, t870: f64, t1914: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6648, t6649, t6650, t6652, t6653, t6654, t6657) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1157(t252, t828, t232, t6646, t1888, t1894, t852, t214, t1880, t1902, t814);
        let (t6658, t6660, t6662) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1158(t6657, t829, t235, t6624, t1909, t226, t6636, t6641, t6645, t6650, t6654, t808, t812);
        let t6663 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1159(t6662, t858);
        let t6665 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1160(t1912, t259, t2597, t2713, t6549, t6557, t6565, t6569, t6574, t6576, t6625, t6627, t6632, t6663, t855, t866);
        let t6666 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1161(t6665, t870);
        let t6670 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1162(t1914, t2752);
    (t6648, t6649, t6652, t6653, t6657, t6658, t6660, t6662, t6663, t6665, t6666, t6670)
}
