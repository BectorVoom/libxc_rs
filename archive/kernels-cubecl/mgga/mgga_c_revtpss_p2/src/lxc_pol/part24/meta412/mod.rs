//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta412<F: Float>(t41741: F, t315: F, t41224: F, t41306: F, t3335: F, t11198: F, t340: F, t338: F, t378: F, t11119: F, t384: F, t225: F) -> (F, F, F, F, F, F, F, F) {
        let (t41742, t41759, t41908, t41937, t42013, t42059, t42060, t42067) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1354::<F>(t41741, t315, t41224, t41306, t3335, t11198, t340, t338, t378, t11119, t384, t225);
    (t41742, t41759, t41908, t41937, t42013, t42059, t42060, t42067)
}
