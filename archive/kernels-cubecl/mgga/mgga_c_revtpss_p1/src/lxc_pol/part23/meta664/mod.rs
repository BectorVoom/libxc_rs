//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta664<F: Float>(t11298: F, t910: F, t41306: F, t3335: F, t11199: F, t988: F, t378: F, t11198: F, t340: F, t338: F, t11119: F, t384: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41883, t41908, t41937, t42013, t42051, t42052, t42059, t42060, t42066) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2395::<F>(t11298, t910, t41306, t3335, t11199, t988, t378, t11198, t340, t338, t11119, t384);
    (t41883, t41908, t41937, t42013, t42051, t42052, t42059, t42060, t42066)
}
