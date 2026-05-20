//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta553<F: Float>(t16756: F, t5333: F, t3720: F, t3588: F, t471: F, t5332: F, t12916: F, t5334: F, t5331: F, t1778: F, t3682: F, t1774: F, t3617: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17780, t17781, t17784, t17785, t17786, t17789, t17791, t17792, t17794) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2377::<F>(t16756, t5333, t3720, t3588, t471, t5332, t12916, t5334, t5331, t1778, t3682, t1774, t3617);
    (t17780, t17781, t17784, t17785, t17786, t17789, t17791, t17792, t17794)
}
