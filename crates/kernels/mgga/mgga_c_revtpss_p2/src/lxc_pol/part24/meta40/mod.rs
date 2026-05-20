//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk287;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk288;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk289;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk290;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta40<F: Float>(t686: F, t874: F, t875: F, t251: F, t822: F, t261: F, t159: F, t675: F, t268: F, t271: F, t373: F, t631: F) -> (F, F, F, F, F, F, F, F) {
        let (t878, t879) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk287::<F>(t686, t874, t875, t251, t822);
        let t892 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk288::<F>(t261);
        let (t900, t902) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk289::<F>(t159, t675, t268, t271);
        let (t903, t904) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk290::<F>(t902, t159, t373);
        let t905 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk291::<F>(t631);
    (t878, t879, t892, t900, t902, t903, t904, t905)
}
