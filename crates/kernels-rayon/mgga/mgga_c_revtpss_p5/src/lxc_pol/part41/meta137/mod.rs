//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk647;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk648;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk649;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk650;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta137(t3719: f64, t828: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t225: f64, t1204: f64, t1284: f64, t487: f64, t1209: f64, t473: f64, t3140: f64, t3596: f64, t3303: f64, t3603: f64, t1243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3720 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk647(t3719, t828);
        let (t3732, t3736, t3737) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk648(t1269, t460, t1275, t493, t225);
        let (t3746, t3754, t3755, t3759, t3766) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk649(t1204, t1284, t487, t1209, t1269, t473, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk650(t3766, t460);
        let (t3769, t3781) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk651(t3303, t3603, t1243, t3140);
    (t3720, t3732, t3736, t3737, t3746, t3754, t3755, t3759, t3766, t3767, t3769, t3781)
}
