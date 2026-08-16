//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk585;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk586;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk587;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta101(t2847: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64, t291: f64, t910: f64, t914: f64, t936: f64, t287: f64, t913: f64, t275: f64, t934: f64, t935: f64, t273: f64, t276: f64, t918: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2866, t2868, t2869, t2871, t2872, t2873, t2874) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk585(t2847, t2848, t2855, t2860, t2864, t291, t910, t914, t936, t287, t913, t275);
        let t2875 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk586(t934);
        let (t2876, t2878, t2880, t2881) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk587(t2875, t935, t2874, t273, t276, t918);
    (t2866, t2868, t2869, t2871, t2872, t2873, t2874, t2875, t2876, t2878, t2880, t2881)
}
