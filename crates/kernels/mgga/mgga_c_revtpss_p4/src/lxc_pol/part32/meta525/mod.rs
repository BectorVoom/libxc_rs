//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta525<F: Float>(t7030: F, t9789: F, t2453: F, t2783: F, t64: F, t10761: F, t9784: F, t2482: F, t25260: F, t27: F, t596: F, t7036: F) -> (F, F, F, F, F, F) {
        let (t93012, t93015, t93016, t93020, t93025, t93034) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1830::<F>(t7030, t9789, t2453, t2783, t64, t10761, t9784, t2482, t25260, t27, t596, t7036);
    (t93012, t93015, t93016, t93020, t93025, t93034)
}
