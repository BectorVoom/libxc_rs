//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta50 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk335;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk336;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk337;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk338;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk339;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta50<F: Float>(t1118: F, t139: F, t221: F, t462: F, t461: F, t1010: F, t56: F, t403: F, t404: F, t1121: F, t1209: F, t225: F, t480: F, t1032: F, t460: F, t472: F, t474: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1212, t1219, t1221, t1222) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk335::<F>(t1118, t139, t221, t462, t461, t1010, t56);
        let t1224 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk336::<F>(t403, t404);
        let (t1225, t1234) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk337::<F>(t1121, t1224, t1209, t225);
        let t1235 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk338::<F>(t1234, t480);
        let (t1241, t1242, t1243) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk339::<F>(t1032, t460, t472);
        let t1244 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk340::<F>(t1243, t474);
    (t1212, t1219, t1221, t1222, t1224, t1225, t1234, t1235, t1241, t1242, t1243, t1244)
}
