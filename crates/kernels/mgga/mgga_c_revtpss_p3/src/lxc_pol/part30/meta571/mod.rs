//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta571<F: Float>(t2466: F, t93329: F, t25375: F, t7015: F, t9292: F, t25411: F, t93183: F, t25431: F, t93123: F, t25387: F, t93285: F, t7063: F, t860: F) -> (F, F, F, F, F, F, F) {
        let (t93330, t93331, t93334, t93335, t93337, t93339, t93341) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2020::<F>(t2466, t93329, t25375, t7015, t9292, t25411, t93183, t25431, t93123, t25387, t93285, t7063, t860);
    (t93330, t93331, t93334, t93335, t93337, t93339, t93341)
}
