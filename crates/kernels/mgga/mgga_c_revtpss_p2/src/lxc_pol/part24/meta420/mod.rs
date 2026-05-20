//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta420 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta420<F: Float>(t43813: F, t43816: F, t3431: F, t408: F, t3434: F, t3800: F, t3362: F, t3603: F, t13100: F, t828: F, t12879: F, t12256: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44039, t44040, t44091, t44093, t44126, t44190, t44225, t44250, t44307, t44348) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1368::<F>(t43813, t43816, t3431, t408, t3434, t3800, t3362, t3603, t13100, t828, t12879, t12256, t3698);
    (t44039, t44040, t44091, t44093, t44126, t44190, t44225, t44250, t44307, t44348)
}
