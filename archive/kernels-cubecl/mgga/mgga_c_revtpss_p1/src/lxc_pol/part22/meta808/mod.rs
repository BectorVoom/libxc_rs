//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta808 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta808<F: Float>(t3869: F, t39538: F, t39427: F, t39535: F, t2496: F, t9551: F, t4038: F, t9372: F, t1317: F, t9428: F, t3853: F, t3857: F) -> (F, F, F, F, F, F, F) {
        let (t47138, t47140, t47142, t47145, t47147, t47149, t47152) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2910::<F>(t3869, t39538, t39427, t39535, t2496, t9551, t4038, t9372, t1317, t9428, t3853, t3857);
    (t47138, t47140, t47142, t47145, t47147, t47149, t47152)
}
