//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk591;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk592;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk593;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk594;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk595;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta97<F: Float>(t22: F, t2224: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F, t25: F, t2219: F, t2221: F, t2223: F, t599: F, t602: F, t89: F, t90: F, t29: F, t644: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk591::<F>(t22, t2224, t584, t588, t20, t27, t12, t19, t592, t596, t21, t25);
        let (t2240, t2242) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk592::<F>(t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t2239, t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk593::<F>(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk594::<F>(t2246, t29);
        let t2248 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk595::<F>(t644);
    (t2226, t2230, t2231, t2233, t2236, t2237, t2239, t2240, t2242, t2246, t2247, t2248)
}
