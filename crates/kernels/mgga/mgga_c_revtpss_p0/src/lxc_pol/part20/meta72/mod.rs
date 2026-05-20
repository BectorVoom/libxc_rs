//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta72 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk464;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk465;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk466;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk467;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk468;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk469;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta72<F: Float>(t576: F, t580: F, t15: F, t22: F, t11: F, t14: F, t584: F, t588: F, t20: F, t27: F, t12: F, t19: F, t592: F, t596: F, t21: F, t25: F, t2219: F, t599: F, t602: F, t89: F, t90: F, t29: F, t644: F, t606: F, t70: F, t2: F, t17: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2221, t2223, t2224, t2226, t2228, t2230, t2231) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk464::<F>(t576, t580, t15, t22, t11, t14, t584, t588, t20, t27, t12, t19);
        let (t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk465::<F>(t2231, t27, t592, t596, t21);
        let t2237 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk466::<F>(t2236);
        let (t2240, t2242) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk467::<F>(t2237, t25, t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t599, t602);
        let (t2246, t2247, t2248) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk468::<F>(t89, t90, t29, t644);
        let t2251 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk469::<F>(t606);
        let (t2252, t2256) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk470::<F>(t2251, t70, t2, t580, t17);
    (t2224, t2231, t2236, t2237, t2240, t2242, t2246, t2247, t2248, t2251, t2252, t2256)
}
