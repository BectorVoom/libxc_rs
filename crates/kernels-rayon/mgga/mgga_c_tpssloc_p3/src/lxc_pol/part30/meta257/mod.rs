//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta257 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1164;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1165;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1166;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1167;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1168;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1169;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1170;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta257(t1930: f64, t6739: f64, t1934: f64, t344: f64, t1009: f64, t1014: f64, t363: f64, t1022: f64, t360: f64, t68: f64, t1004: f64, t1941: f64, sigma0: f64, t1018: f64, t1012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6740 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1164(t1930, t6739);
        let (t6741, t6742) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1165(t1934, t344, t6740);
        let t6743 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1166(t1009, t1014);
        let t6744 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1167(t363, t6743);
        let (t6746, t6747) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1168(t1022, t360, t68, t6744);
        let (t6750, t6753) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1169(t1004, t1941, t1014, sigma0);
        let t6754 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1170(t1018, t6753);
        let t6755 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1171(t1012, t6754);
    (t6740, t6741, t6742, t6743, t6744, t6746, t6747, t6750, t6753, t6754, t6755)
}
