//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta17 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk134;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk135;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk136;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk137;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk138;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk139;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk140;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta17(t315: f64, t324: f64, t293: f64, t300: f64, t302: f64, t311: f64, t199: f64, t240: f64, zeta_threshold: f64, t273: f64, t136: f64, t44: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t328, t330, t334, t335) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk134(t315, t324, t293, t300, t302, t311, t199, t240, zeta_threshold);
        let t336 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk135(t334, t335);
        let t338 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk136(t273);
        let (t340, t341) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk137(t273);
        let t342 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk138(t338, t341);
        let t344 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk139(t335, t136);
        let t345 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk140(t344, t44);
        let t346 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk141(t271);
    (t328, t330, t334, t335, t336, t338, t340, t341, t342, t344, t345, t346)
}
