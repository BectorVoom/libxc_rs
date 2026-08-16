//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk742;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk743;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk744;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk745;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta166<F: Float>(t3584: F, t482: F, t371: F, t372: F, t225: F, t3555: F, t480: F, t3566: F, t3568: F, t1236: F, t127: F, t1235: F, t221: F, t462: F, t696: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3661, t3663, t3666) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk742::<F>(t3584, t482, t371, t372, t225, t3555);
        let t3667 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk743::<F>(t3666, t480);
        let t3670 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk744::<F>(t225, t3566);
        let (t3671, t3672, t3674, t3678, t3679, t3682) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk745::<F>(t3670, t480, t3568, t482, t371, t372, t1236, t127, t1235, t221, t462, t696);
    (t3661, t3663, t3666, t3667, t3670, t3671, t3672, t3674, t3678, t3679, t3682)
}
