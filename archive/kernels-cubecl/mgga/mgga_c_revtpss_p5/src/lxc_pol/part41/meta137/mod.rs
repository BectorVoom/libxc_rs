//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta137 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk647;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk648;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk649;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk650;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta137<F: Float>(t3719: F, t828: F, t1269: F, t460: F, t1275: F, t493: F, t225: F, t1204: F, t1284: F, t487: F, t1209: F, t473: F, t3140: F, t3596: F, t3303: F, t3603: F, t1243: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3720 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk647::<F>(t3719, t828);
        let (t3732, t3736, t3737) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk648::<F>(t1269, t460, t1275, t493, t225);
        let (t3746, t3754, t3755, t3759, t3766) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk649::<F>(t1204, t1284, t487, t1209, t1269, t473, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk650::<F>(t3766, t460);
        let (t3769, t3781) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk651::<F>(t3303, t3603, t1243, t3140);
    (t3720, t3732, t3736, t3737, t3746, t3754, t3755, t3759, t3766, t3767, t3769, t3781)
}
