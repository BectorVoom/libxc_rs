//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk631;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk632;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta133(t3566: f64, t487: f64, t1209: f64, t1269: f64, t3356: f64, t3140: f64, t460: f64, t1242: f64, t472: f64, t474: f64, t3147: f64, t479: f64, t471: f64, t3153: f64, t1244: f64, t1121: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3567, t3572, t3579, t3594, t3596, t3597, t3598) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk631(t3566, t487, t1209, t1269, t3356, t3140, t460, t1242, t472, t474, t3147, t479);
        let (t3599, t3600, t3603) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk632(t3597, t3598, t3594, t471);
        let (t3604, t3609, t3610, t3611, t3617) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk633(t3153, t3603, t1244, t3598, t3594, t471, t1121, t414);
    (t3567, t3572, t3579, t3596, t3599, t3600, t3603, t3604, t3609, t3610, t3611, t3617)
}
