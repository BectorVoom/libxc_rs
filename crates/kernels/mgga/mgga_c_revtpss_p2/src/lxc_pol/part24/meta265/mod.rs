//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta265 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta265<F: Float>(t13148: F, t17708: F, t1209: F, t489: F, t3623: F, t370: F, t3566: F, t13142: F, t13127: F, t1778: F, t3682: F, t372: F, t5268: F) -> (F, F, F, F, F, F, F) {
        let (t17709, t17729, t17736, t17747, t17753, t17792, t17799) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1037::<F>(t13148, t17708, t1209, t489, t3623, t370, t3566, t13142, t13127, t1778, t3682, t372, t5268);
    (t17709, t17729, t17736, t17747, t17753, t17792, t17799)
}
