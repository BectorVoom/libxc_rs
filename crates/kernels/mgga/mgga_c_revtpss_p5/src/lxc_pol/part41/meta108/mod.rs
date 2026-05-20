//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta108 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk563;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta108<F: Float>(t136: F, t854: F, t221: F, t775: F, t2674: F, t26: F, t66: F, t240: F, t243: F, t247: F, t237: F, t124: F, t212: F, t596: F, t800: F) -> (F, F, F, F, F, F, F) {
        let (t2675, t2677, t2678, t2681) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk563::<F>(t136, t854, t221, t775, t2674, t26, t66);
        let (t2682, t2686, t2689) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk564::<F>(t240, t2681, t243, t247, t237, t124, t212, t596, t800);
    (t2675, t2677, t2678, t2681, t2682, t2686, t2689)
}
