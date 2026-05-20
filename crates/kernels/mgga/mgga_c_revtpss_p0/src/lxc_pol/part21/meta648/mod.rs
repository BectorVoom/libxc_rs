//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta648<F: Float>(t11199: F, t988: F, t378: F, t11198: F, t340: F, t338: F, t11119: F, t384: F, t225: F, t41306: F, t3057: F, t3259: F) -> (F, F, F, F, F, F, F) {
        let (t42051, t42052, t42059, t42060, t42067, t42078, t42107) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2433::<F>(t11199, t988, t378, t11198, t340, t338, t11119, t384, t225, t41306, t3057, t3259);
    (t42051, t42052, t42059, t42060, t42067, t42078, t42107)
}
