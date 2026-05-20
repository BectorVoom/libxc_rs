//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta357<F: Float>(t14732: F, t2484: F, t2652: F, t4435: F, t4343: F, t854: F, t236: F, t807: F, t221: F, t4433: F, t10703: F, t2674: F) -> (F, F, F, F, F, F, F) {
        let (t14734, t14736, t14741, t14744, t14756, t14757, t14759) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1299::<F>(t14732, t2484, t2652, t4435, t4343, t854, t236, t807, t221, t4433, t10703, t2674);
    (t14734, t14736, t14741, t14744, t14756, t14757, t14759)
}
