//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1142;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1143;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1144;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1145;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta253<F: Float>(t252: F, t828: F, t232: F, t6646: F, t1888: F, t1894: F, t852: F, t214: F, t1880: F, t1902: F, t814: F, t829: F, t235: F, t6624: F, t1909: F, t226: F, t6636: F, t6641: F, t6645: F, t808: F, t812: F, t858: F, t1912: F, t259: F, t2597: F, t2713: F, t6549: F, t6557: F, t6565: F, t6569: F, t6574: F, t6576: F, t6625: F, t6627: F, t6632: F, t855: F, t866: F, t870: F, t1914: F, t2752: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6648, t6649, t6650, t6652, t6653, t6654, t6657) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1141::<F>(t252, t828, t232, t6646, t1888, t1894, t852, t214, t1880, t1902, t814);
        let (t6658, t6660, t6662) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1142::<F>(t6657, t829, t235, t6624, t1909, t226, t6636, t6641, t6645, t6650, t6654, t808, t812);
        let t6663 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1143::<F>(t6662, t858);
        let t6665 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1144::<F>(t1912, t259, t2597, t2713, t6549, t6557, t6565, t6569, t6574, t6576, t6625, t6627, t6632, t6663, t855, t866);
        let t6666 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1145::<F>(t6665, t870);
        let t6670 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1146::<F>(t1914, t2752);
    (t6648, t6649, t6652, t6653, t6657, t6658, t6660, t6662, t6663, t6665, t6666, t6670)
}
