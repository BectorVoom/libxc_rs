//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta490<F: Float>(t1399: F, t676: F, t25880: F, t25899: F, t25894: F, t25898: F) -> (F, F, F, F) {
        let (t25900, t25901, t25902, t25904) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1787::<F>(t1399, t676, t25880, t25899, t25894, t25898);
    (t25900, t25901, t25902, t25904)
}
