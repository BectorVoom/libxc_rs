//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta169 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk750;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk751;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk752;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk753;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta169<F: Float>(t3719: F, t828: F, t1214: F, t1248: F, t1250: F, t1222: F, t1235: F, t1238: F, t1252: F, t3663: F, t3667: F, t3671: F, t3674: F, t3679: F, t3684: F, t3686: F, t3689: F, t3694: F, t3701: F, t3705: F, t3708: F, t3711: F, t3714: F, t3718: F, t3660: F, t225: F, t494: F, t1269: F, t460: F, t1275: F, t493: F, t1294: F, t1204: F, t1284: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3720 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk750::<F>(t3719, t828);
        let (t3721, t3722, t3723, t3726) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk751::<F>(t1214, t1248, t1250, t3720, t1222, t1235, t1238, t1252, t3663, t3667, t3671, t3674, t3679, t3684, t3686, t3689, t3694, t3701, t3705, t3708, t3711, t3714, t3718);
        let t3727 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk752::<F>(t3660, t3726);
        let (t3729, t3732, t3736, t3737) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk753::<F>(t225, t3727, t494, t1269, t460, t1275, t493);
        let (t3738, t3739, t3746) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk754::<F>(t1294, t3737, t1204, t1284);
    (t3720, t3721, t3722, t3723, t3727, t3729, t3732, t3736, t3737, t3738, t3739, t3746)
}
