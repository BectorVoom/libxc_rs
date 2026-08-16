//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta132 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk694;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk695;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk696;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk697;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk698;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk699;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk700;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta132<F: Float>(t1263: F, t1774: F, t1038: F, t1802: F, t1244: F, t1241: F, t1121: F, t3362: F, t3617: F, t1012: F, t1224: F, t3698: F, t1234: F, t1803: F, t225: F, t5219: F, t480: F, t3623: F, t4890: F, t3782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5277, t5292, t5293) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk694::<F>(t1263, t1774, t1038, t1802, t1244, t1241);
        let (t5296, t5302) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk695::<F>(t1121, t1263, t3362, t3617);
        let (t5308, t5312, t5323) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk696::<F>(t1012, t1224, t3698, t1234, t1803);
        let t5326 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk697::<F>(t225, t5219);
        let t5327 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk698::<F>(t480, t5326);
        let t5330 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk699::<F>(t3623, t4890);
        let t5331 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk700::<F>(t3782, t5330);
    (t5277, t5292, t5293, t5296, t5302, t5308, t5312, t5323, t5326, t5327, t5330, t5331)
}
