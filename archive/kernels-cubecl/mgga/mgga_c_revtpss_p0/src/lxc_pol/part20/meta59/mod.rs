//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta59 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk398;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk399;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk400;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk401;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk402;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk403;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta59<F: Float>(t1209: F, t487: F, t225: F, t494: F, t1118: F, t1124: F, t139: F, t221: F, t462: F, t461: F, t1010: F, t56: F, t403: F, t404: F) -> (F, F, F, F, F, F, F) {
        let t1210 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk398::<F>(t1209, t487);
        let t1211 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk399::<F>(t225, t494);
        let t1214 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk400::<F>(t1118, t1124);
        let t1215 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk401::<F>(t1211, t1214);
        let (t1221, t1222) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk402::<F>(t139, t221, t462, t461, t1010, t56);
        let t1224 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk403::<F>(t403, t404);
    (t1210, t1211, t1214, t1215, t1221, t1222, t1224)
}
