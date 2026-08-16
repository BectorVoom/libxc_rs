//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk464;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk465;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk466;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk467;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk468;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk469;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta72(t576: f64, t580: f64, t15: f64, t22: f64, t11: f64, t14: f64, t584: f64, t588: f64, t20: f64, t27: f64, t12: f64, t19: f64, t592: f64, t596: f64, t21: f64, t25: f64, t2219: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t644: f64, t606: f64, t70: f64, t2: f64, t17: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2221, t2223, t2224, t2226, t2228, t2230, t2231) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk464(t576, t580, t15, t22, t11, t14, t584, t588, t20, t27, t12, t19);
        let (t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk465(t2231, t27, t592, t596, t21);
        let t2237 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk466(t2236);
        let (t2240, t2242) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk467(t2237, t25, t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t599, t602);
        let (t2246, t2247, t2248) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk468(t89, t90, t29, t644);
        let t2251 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk469(t606);
        let (t2252, t2256) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk470(t2251, t70, t2, t580, t17);
    (t2224, t2231, t2236, t2237, t2240, t2242, t2246, t2247, t2248, t2251, t2252, t2256)
}
