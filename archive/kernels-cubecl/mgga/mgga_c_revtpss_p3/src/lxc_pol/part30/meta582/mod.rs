//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta582<F: Float>(t25877: F, t94390: F, t94385: F, t9675: F, t7289: F, t94377: F, t122: F, t72: F, t7274: F, t3916: F, t25895: F, t7285: F, t9288: F) -> (F, F, F, F, F, F, F, F) {
        let (t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2036::<F>(t25877, t94390, t94385, t9675, t7289, t94377, t122, t72, t7274, t3916, t25895, t7285, t9288);
    (t94589, t94590, t94591, t94593, t94596, t94597, t94598, t94600)
}
