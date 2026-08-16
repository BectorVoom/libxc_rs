//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk617;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk618;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk619;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk620;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk621;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta97<F: Float>(t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F, t25: F, t2219: F, t2221: F, t2223: F, t2226: F, t2228: F, t599: F, t602: F, t89: F, t90: F, t29: F, t644: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2230, t2231, t2233, t2236, t2237, t2239, t2240) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk617::<F>(t20, t27, t12, t19, t592, t596, t21, t25, t2219, t2221, t2223, t2226, t2228);
        let t2242 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk618::<F>(t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk619::<F>(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk620::<F>(t2246, t29);
        let t2248 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk621::<F>(t644);
        let t2251 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk622::<F>(t606);
    (t2230, t2231, t2233, t2236, t2237, t2239, t2240, t2242, t2246, t2247, t2248, t2251)
}
