//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk453;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk454;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk455;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk456;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta75<F: Float>(t117: F, t1501: F, t1468: F, t100: F, t55: F, tau1: F, t108: F, t105: F, t109: F, t97: F, t114: F, t655: F, t653: F, t69: F) -> (F, F, F, F, F, F, F, F, F) {
        let t1502 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk453::<F>(t117, t1501);
        let t1504 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk454::<F>(t1468);
        let (t1505, t1507, t1509) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk455::<F>(t100, t1504, t55, tau1);
        let (t1510, t1513) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk456::<F>(t108, t1509, t105, t109, t1505, t1507, t97);
        let (t1514, t1518) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk457::<F>(t114, t1513, t655, t653, t69);
    (t1502, t1504, t1505, t1507, t1509, t1510, t1513, t1514, t1518)
}
