//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk962;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk963;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk964;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk965;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta158(t231: f64, t2783: f64, t4494: f64, t2782: f64, t1559: f64, t72: f64, t686: f64, t2798: f64, t225: f64, t2718: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4496 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk962(t231, t2783, t4494);
        let (t4497, t4499, t4500) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk963(t2782, t4496, t1559, t72, t686);
        let (t4501, t4503) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk964(t2798, t4500, t225, t2718);
        let t4504 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk965(t213, t4503);
        let t4514 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk966(t213, t2783);
    (t4496, t4497, t4499, t4500, t4501, t4503, t4504, t4514)
}
