//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk493;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta85<F: Float>(t1120: F, t1715: F, t128: F, t1119: F, t422: F, t1118: F, t1132: F, t1139: F, t1145: F, t141: F, t1137: F, t1144: F, t1150: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1716, t1717, t1719, t1721, t1723) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk493::<F>(t1120, t1715, t128, t1119, t422, t1118);
        let (t1724, t1727, t1729, t1730, t1732, t1733) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk494::<F>(t1132, t1723, t1139, t1145, t1715, t141, t1137, t1144, t1717, t1150);
    (t1716, t1717, t1719, t1721, t1723, t1724, t1727, t1729, t1730, t1732, t1733)
}
