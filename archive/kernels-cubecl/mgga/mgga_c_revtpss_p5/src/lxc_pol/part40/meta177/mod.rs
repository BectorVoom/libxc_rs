//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta177 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta177<F: Float>(t136: F, t561: F, t2457: F, t3906: F, t1420: F, t786: F, t1364: F, t1426: F, t556: F) -> (F, F, F, F, F, F, F) {
        let (t3907, t3908, t3910, t3911, t3912, t3914, t3915) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk772::<F>(t136, t561, t2457, t3906, t1420, t786, t1364, t1426, t556);
    (t3907, t3908, t3910, t3911, t3912, t3914, t3915)
}
