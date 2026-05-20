//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1127;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta332<F: Float>(t240: F, t849: F, t14648: F, t775: F, t2661: F, t2652: F, t4345: F, t10716: F, t4349: F, t2689: F, t4372: F, t4354: F, t9775: F, t221: F, t2675: F, t4343: F, t2674: F, t243: F, t4423: F, t231: F, t2662: F, t10722: F, t1565: F, t4352: F, t4366: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14836, t14837, t14839, t14846, t14850) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1127::<F>(t240, t849, t14648, t775, t2661, t2652, t4345, t10716, t4349, t2689, t4372, t4354, t9775);
        let (t14859, t14864, t14866, t14868) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1128::<F>(t221, t2675, t4343, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366);
    (t14836, t14837, t14839, t14846, t14850, t14859, t14864, t14866, t14868)
}
