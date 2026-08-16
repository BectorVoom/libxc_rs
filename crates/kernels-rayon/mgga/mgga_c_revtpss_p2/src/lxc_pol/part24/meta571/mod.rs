//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta571(t6470: f64, t1150: f64, t3384: f64, t3433: f64, t3435: f64, t1733: f64, t81146: f64, t20629: f64, t6471: f64, t6439: f64, t90293: f64, t90321: f64, t90323: f64, t90327: f64, t90329: f64, t90332: f64, t17092: f64, t24212: f64, t16840: f64, t24215: f64, t6534: f64, t1196: f64, t3520: f64, t3523: f64, t6518: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90336, t90339, t90341, t90343, t90346, t90347) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749(t6470, t1150, t3384, t3433, t3435, t1733, t81146, t20629, t6471, t6439, t90293, t90321, t90323, t90327, t90329, t90332);
        let (t90349, t90351, t90352, t90356, t90357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750(t17092, t24212, t16840, t24215, t6534, t1196, t3520, t3523, t6518);
    (t90336, t90339, t90341, t90343, t90346, t90347, t90349, t90351, t90352, t90356, t90357)
}
