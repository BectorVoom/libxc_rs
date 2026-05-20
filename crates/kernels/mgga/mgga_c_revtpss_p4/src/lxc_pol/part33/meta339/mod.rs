//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta339<F: Float>(t3618: F, t828: F, t1209: F, t3781: F, t5330: F, t1284: F, t3555: F, t3624: F, t1121: F, t3603: F, t606: F, t221: F, t462: F, t68: F) -> (F, F, F, F, F, F, F) {
        let (t12787, t12808, t12809, t12831, t12832, t12840, t12851) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1350::<F>(t3618, t828, t1209, t3781, t5330, t1284, t3555, t3624, t1121, t3603, t606, t221, t462, t68);
    (t12787, t12808, t12809, t12831, t12832, t12840, t12851)
}
