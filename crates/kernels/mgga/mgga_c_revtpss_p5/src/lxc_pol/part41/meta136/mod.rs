//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk643;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk644;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk645;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta136<F: Float>(t1222: F, t3685: F, t1224: F, t3367: F, t1121: F, t404: F, t3362: F, t1251: F, t3172: F, t1247: F, t1032: F, t1204: F, t1246: F, t1234: F, t1260: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3686, t3692, t3698, t3699, t3704, t3705, t3707) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk643::<F>(t1222, t3685, t1224, t3367, t1121, t404, t3362, t1251, t3172, t1247, t1032, t1204);
        let (t3708, t3711) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk644::<F>(t1246, t3707, t1234, t1260);
        let (t3717, t3718) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk645::<F>(t1209, t1284, t3624);
        let t3719 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk646::<F>(t482, t66);
    (t3686, t3692, t3698, t3699, t3704, t3705, t3707, t3708, t3711, t3717, t3718, t3719)
}
