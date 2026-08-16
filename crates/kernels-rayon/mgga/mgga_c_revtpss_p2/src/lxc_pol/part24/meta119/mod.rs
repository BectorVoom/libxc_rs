//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk655;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta119(t2662: f64, t4353: f64, t2661: f64, t1565: f64, t2652: f64, t1561: f64, t2741: f64, t241: f64, t2719: f64, t820: f64, t243: f64, t72: f64, t245: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4354, t4355, t4357, t4359, t4362, t4363) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk655(t2662, t4353, t2661, t1565, t2652, t1561, t2741, t241, t2719, t820, t243, t72);
        let t4364 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk656(t245, t4363);
    (t4354, t4355, t4357, t4359, t4362, t4364)
}
