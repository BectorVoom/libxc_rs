//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk661;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta123<F: Float>(t2782: F, t4496: F, t1559: F, t72: F, t686: F, t2798: F, t225: F, t2718: F) -> (F, F, F, F, F) {
        let (t4497, t4499, t4500, t4501, t4503) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk661::<F>(t2782, t4496, t1559, t72, t686, t2798, t225, t2718);
    (t4497, t4499, t4500, t4501, t4503)
}
