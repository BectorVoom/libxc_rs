//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta119 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk597;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk598;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk599;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta119<F: Float>(t2724: F, t827: F, t828: F, t159: F, t243: F, t216: F, t124: F, t2394: F, t800: F, t2712: F, t785: F, t225: F, t826: F, t849: F, t820: F, t823: F, t843: F, t839: F, t241: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2726, t2729, t2730, t2732, t2735) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk597::<F>(t2724, t827, t828, t159, t243, t216, t124, t2394, t800, t2712, t785);
        let (t2736, t2737, t2739, t2741) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk598::<F>(t225, t2735, t826, t849, t820, t823, t843);
        let (t2742, t2745) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk599::<F>(t2741, t839, t241, t820, t823);
    (t2726, t2729, t2730, t2732, t2735, t2736, t2737, t2739, t2741, t2742, t2745)
}
