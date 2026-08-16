//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta247 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1529;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1530;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1531;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1532;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1533;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1534;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta247<F: Float>(t6141: F, t935: F, t915: F, t2926: F, t6109: F, t2924: F, t2930: F, t4571: F, t6094: F, t6098: F, t6102: F, t1621: F, t954: F, t2950: F, t2957: F, t4620: F, t6114: F, t6121: F, t6127: F, t6129: F, t6133: F, t6136: F, t6139: F, t2970: F, t2974: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6142 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1529::<F>(t6141, t935);
        let (t6144, t6145) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1530::<F>(t6142, t915, t2926, t6109);
        let (t6147, t6152, t6157, t6158) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1531::<F>(t2924, t6145, t2930, t4571, t6094, t6098, t6102, t1621, t954);
        let t6173 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1532::<F>(t2950, t2957, t4571, t4620, t6094, t6098, t6102, t6114, t6121, t6127, t6129, t6133, t6136, t6139);
        let t6174 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1533::<F>(t6173, t954);
        let t6177 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1534::<F>(t2970, t6157);
        let t6184 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1535::<F>(t2974, t4571, t6094, t6098, t6102);
    (t6142, t6144, t6145, t6147, t6152, t6157, t6158, t6173, t6174, t6177, t6184)
}
