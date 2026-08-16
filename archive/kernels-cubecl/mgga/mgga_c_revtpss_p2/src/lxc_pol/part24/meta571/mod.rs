//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta571<F: Float>(t6470: F, t1150: F, t3384: F, t3433: F, t3435: F, t1733: F, t81146: F, t20629: F, t6471: F, t6439: F, t90293: F, t90321: F, t90323: F, t90327: F, t90329: F, t90332: F, t17092: F, t24212: F, t16840: F, t24215: F, t6534: F, t1196: F, t3520: F, t3523: F, t6518: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90336, t90339, t90341, t90343, t90346, t90347) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1749::<F>(t6470, t1150, t3384, t3433, t3435, t1733, t81146, t20629, t6471, t6439, t90293, t90321, t90323, t90327, t90329, t90332);
        let (t90349, t90351, t90352, t90356, t90357) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1750::<F>(t17092, t24212, t16840, t24215, t6534, t1196, t3520, t3523, t6518);
    (t90336, t90339, t90341, t90343, t90346, t90347, t90349, t90351, t90352, t90356, t90357)
}
