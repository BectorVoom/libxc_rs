//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta593<F: Float>(t46361: F, t545: F, t25880: F, t9685: F, t25895: F, t25900: F, t94596: F, t25904: F, t1032: F, t9656: F, t25875: F, t25925: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F) {
        let (t94656, t94661, t94662, t94664, t94665, t94668, t94669, t94671) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2064::<F>(t46361, t545, t25880, t9685, t25895, t25900, t94596, t25904, t1032, t9656, t25875, t25925, t686, t72);
    (t94656, t94661, t94662, t94664, t94665, t94668, t94669, t94671)
}
