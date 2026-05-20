//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk575;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk576;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk577;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta111<F: Float>(t2741: F, t839: F, t241: F, t820: F, t823: F, t72: F, t853: F, t245: F, t231: F, t775: F, t213: F, t860: F, t256: F, t866: F, t225: F, t2435: F, t871: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk575::<F>(t2741, t839, t241, t820, t823);
        let (t2746, t2747) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk576::<F>(t72, t853, t245);
        let t2749 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk577::<F>(t231, t775);
        let (t2765, t2769, t2770, t2776, t2777) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk578::<F>(t213, t860, t256, t866, t225, t2435, t871, t785);
    (t2742, t2745, t2746, t2747, t2749, t2765, t2769, t2770, t2776, t2777)
}
