//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta938 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta938<F: Float>(t12227: F, t1732: F, t12248: F, t3433: F, t16831: F, t300: F, t12429: F, t1744: F, t12472: F, t5142: F, t17150: F, t3523: F) -> (F, F, F, F, F, F, F) {
        let (t57795, t57818, t57854, t57861, t57944, t57972, t58000) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3173::<F>(t12227, t1732, t12248, t3433, t16831, t300, t12429, t1744, t12472, t5142, t17150, t3523);
    (t57795, t57818, t57854, t57861, t57944, t57972, t58000)
}
