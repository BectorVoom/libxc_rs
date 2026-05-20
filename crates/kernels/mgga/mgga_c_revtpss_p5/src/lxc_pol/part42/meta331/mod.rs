//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1125;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1126;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta331<F: Float>(t10845: F, t4430: F, t1558: F, t853: F, t2749: F, t2662: F, t2661: F, t4352: F, t837: F, t4416: F, t221: F, t2485: F, t4424: F, t2484: F, t2652: F, t4435: F, t4343: F, t854: F, t236: F, t807: F, t4433: F, t10703: F, t2674: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14716, t14718, t14722, t14726, t14730, t14732) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1125::<F>(t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837, t4416, t221, t2485, t4424);
        let (t14734, t14736, t14744, t14759) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1126::<F>(t14732, t2484, t2652, t4435, t4343, t854, t236, t807, t221, t4433, t10703, t2674);
    (t14716, t14718, t14722, t14726, t14730, t14734, t14736, t14744, t14759)
}
