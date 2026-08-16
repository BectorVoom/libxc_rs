//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1170;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta357<F: Float>(t3678: F, t5327: F, t5323: F, t3667: F, t5362: F, t1789: F, t371: F, t676: F, t1235: F, t1769: F, t3565: F, t225: F) -> (F, F, F, F, F, F) {
        let (t17296, t17298, t17301, t17304, t17306, t17307) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1170::<F>(t3678, t5327, t5323, t3667, t5362, t1789, t371, t676, t1235, t1769, t3565, t225);
    (t17296, t17298, t17301, t17304, t17306, t17307)
}
