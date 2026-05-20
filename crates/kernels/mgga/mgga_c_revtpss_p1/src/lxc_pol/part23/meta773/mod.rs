//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta773<F: Float>(t57726: F, t12248: F, t1732: F, t3433: F, t56176: F, t56183: F, t56228: F, t12429: F, t1744: F, t12469: F, t1737: F, t3362: F, t462: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t57727, t57818, t57854, t57872, t57874, t57889, t57944, t58005, t58027) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2577::<F>(t57726, t12248, t1732, t3433, t56176, t56183, t56228, t12429, t1744, t12469, t1737, t3362, t462);
    (t57727, t57818, t57854, t57872, t57874, t57889, t57944, t58005, t58027)
}
