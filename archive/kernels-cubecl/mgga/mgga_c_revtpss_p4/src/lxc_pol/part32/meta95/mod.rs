//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk588;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk589;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk590;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta95<F: Float>(t22: F, t2224: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F, t25: F, t599: F, t602: F, t89: F, t90: F, t29: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk588::<F>(t22, t2224, t584, t588, t20, t27, t12, t19, t592, t596, t21, t25);
        let t2242 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk589::<F>(t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk590::<F>(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk591::<F>(t2246, t29);
    (t2226, t2228, t2230, t2231, t2233, t2235, t2236, t2237, t2239, t2242, t2246, t2247)
}
