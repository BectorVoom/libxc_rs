//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta29 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk222;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk223;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk224;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk225;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk226;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta29(t11: f64, t583: f64, t22: f64, t21: f64, t3: f64, t20: f64, t12: f64, t19: f64, t2: f64, t27: f64, t579: f64, t25: f64, t578: f64, t582: f64, t88: f64, t90: f64, t29: f64, t17: f64, t4: f64, t30: f64, t33: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t584, t586, t587, t588) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk222(t11, t583, t22, t21, t3);
        let (t590, t592, t594, t595, t596) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk223(t20, t588, t12, t19, t2, t27, t21, t579);
        let (t599, t602, t603, t604) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk224(t25, t596, t578, t582, t586, t590, t594, t88, t90, t29, t17, t2);
        let t605 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk225(t4, t604);
        let t606 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk226(t30, t33, t605, zeta_threshold);
    (t584, t587, t588, t592, t595, t596, t599, t602, t603, t605, t606)
}
