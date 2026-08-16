//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta15 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk123;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk124;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk125;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk126;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk127;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta15(t268: f64, t269: f64, t271: f64, t124: f64, t138: f64, t139: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t273 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk123(t268, t269, t271);
        let t275 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk124(t273);
        let t276 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk125(t273);
        let (t279, t281) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk126(t273, t124, t138);
        let (t282, t283) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk127(t139, t240, t271);
        let (t285, t287, t290, t291) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk128(t281, t282, t283, t273, t276, t279);
    (t273, t275, t276, t279, t281, t282, t283, t285, t287, t290, t291)
}
