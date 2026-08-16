//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2128;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2130;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta619(t2033: f64, t3829: f64, t2014: f64, t7900: f64, t28067: f64, t95088: f64, t14468: f64, t30: f64, t2: f64, t2411: f64, t580: f64, t890: f64, t27382: f64, t198: f64, t206: f64, t7782: f64, t892: f64, t775: f64, t25206: f64, t1583: f64, t2430: f64, t25207: f64, t1468: f64, t14365: f64, t1544: f64, t2257: f64, t205: f64, t7086: f64, t4433: f64, t1940: f64, t1963: f64, t2403: f64, t25198: f64, t25208: f64, t25449: f64, t27158: f64, t27160: f64, t27169: f64, t27364: f64, t27368: f64, t27395: f64, t4541: f64, t605: f64, t7087: f64, t7783: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98621, t98623, t98627, t98633) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2128(t2033, t3829, t2014, t7900, t28067, t95088, t14468, t30, t2, t2411, t580, t890);
        let (t98635, t98637, t98650, t98651) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2129(t27382, t98633, t198, t206, t7782, t2, t892, t580, t775, t25206, t1583, t2430);
        let (t98652, t98659, t98662, t98669, t98674) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2130(t25207, t98651, t1468, t2411, t14365, t1544, t2257, t198, t205, t7086, t4433, t890);
        let t98678 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2131(t25207, t98674, t1940, t1963, t2403, t25198, t25206, t25208, t25449, t27158, t27160, t27169, t27364, t27368, t27395, t4541, t605, t7087, t7783, t98627, t98635, t98637, t98650, t98652, t98659, t98662, t98669);
    (t98621, t98623, t98635, t98637, t98650, t98651, t98669, t98674, t98678)
}
