//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1241;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1242;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1243;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1244;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta273<F: Float>(t1484: F, t6638: F, t6637: F, t6552: F, t232: F, t4282: F, t6646: F, t1888: F, t1519: F, t1894: F, t214: F, t1880: F, t1510: F, t6657: F, t235: F, t7510: F, t1499: F, t1909: F, t226: F, t6636: F, t6645: F, t812: F, t858: F, t1528: F, t1912: F, t259: F, t4147: F, t4268: F, t6549: F, t6565: F, t6627: F, t7481: F, t7486: F, t7490: F, t7492: F, t7511: F, t7517: F, t855: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7520, t7521, t7522, t7524, t7525, t7526, t7528, t7529, t7530) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1241::<F>(t1484, t6638, t6637, t6552, t232, t4282, t6646, t1888, t1519, t1894, t214, t1880);
        let (t7533, t7535, t7537) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1242::<F>(t1510, t6657, t235, t7510, t1499, t1909, t226, t6636, t6645, t7522, t7526, t7530, t812);
        let t7538 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1243::<F>(t7537, t858);
        let t7540 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1244::<F>(t1528, t1912, t259, t4147, t4268, t6549, t6565, t6627, t7481, t7486, t7490, t7492, t7511, t7517, t7538, t855);
        let t7541 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1245::<F>(t7540, t870);
    (t7520, t7521, t7524, t7525, t7528, t7529, t7533, t7535, t7537, t7538, t7540, t7541)
}
