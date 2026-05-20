//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta248 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1536;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1537;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1538;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1539;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1540;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta248<F: Float>(t324: F, t6184: F, t1633: F, t973: F, t2994: F, t3001: F, t4571: F, t4620: F, t6094: F, t6098: F, t6102: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F, t3014: F, t1622: F, t1634: F, t2943: F, t2968: F, t2987: F, t3012: F, t311: F, t4647: F, t4685: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6152: F, t6158: F, t6174: F, t6177: F, t946: F, t965: F) -> (F, F, F, F, F, F, F) {
        let (t6185, t6189) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1536::<F>(t324, t6184, t1633);
        let t6190 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1537::<F>(t6189, t973);
        let t6205 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1538::<F>(t2994, t3001, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
        let t6206 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1539::<F>(t6205, t973);
        let t6209 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1540::<F>(t3014, t6189);
        let t6212 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1541::<F>(t1622, t1634, t2943, t2968, t2987, t3012, t311, t4647, t4685, t6106, t6108, t6112, t6144, t6147, t6152, t6158, t6174, t6177, t6185, t6190, t6206, t6209, t946, t965);
    (t6185, t6189, t6190, t6205, t6206, t6209, t6212)
}
