//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1908;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1909;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1910;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta368(t12898: f64, t481: f64, t3172: f64, t3605: f64, t3600: f64, t11262: f64, t1251: f64, t1247: f64, t3704: f64, t3708: f64, t1284: f64, t3566: f64, t3624: f64, t126: f64, t482: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12900, t12901, t12902, t12904, t12905, t12907, t12909) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1908(t12898, t481, t3172, t3605, t3600, t11262, t1251, t1247, t3704, t3708, t1284, t3566);
        let t12910 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1909(t12909, t3624);
        let t12915 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1910(t126, t482);
        let t12916 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1911(t12915, t828);
    (t12900, t12901, t12902, t12904, t12905, t12907, t12909, t12910, t12915, t12916)
}
