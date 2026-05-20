//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1288;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1289;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1290;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1291;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1292;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1293;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1294;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta203<F: Float>(t3088: F, t4890: F, t3299: F, t1668: F, t3153: F, t1043: F, t3154: F, t3117: F, t3317: F, t357: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t4891 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1288::<F>(t3088, t4890);
        let t4892 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1289::<F>(t3299, t4891);
        let t4893 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1290::<F>(t1668, t3153);
        let t4894 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1291::<F>(t1043, t3154);
        let (t4895, t4896) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1292::<F>(t4893, t4894, t3117);
        let t4899 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1293::<F>(t3317, t4891);
        let t4900 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1294::<F>(t1043, t357);
        let (t4901, t4902) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1295::<F>(t4893, t4900, t3117);
    (t4891, t4892, t4893, t4894, t4895, t4896, t4899, t4900, t4901, t4902)
}
