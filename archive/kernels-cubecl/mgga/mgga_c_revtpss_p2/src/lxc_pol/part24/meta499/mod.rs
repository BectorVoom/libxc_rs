//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta499<F: Float>(t14598: F, t23160: F, t686: F, t72: F, t23244: F, t251: F, t1568: F, t5977: F, t2723: F, t2782: F, t4503: F, t1558: F, t6041: F, t231: F, t2783: F, t4500: F, t62967: F, t23168: F, t39598: F, t10530: F, t23172: F) -> (F, F, F, F, F, F, F, F) {
        let (t76125, t76127, t76131, t76134, t76136) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1501::<F>(t14598, t23160, t686, t72, t23244, t251, t1568, t5977, t2723, t2782, t4503, t1558, t6041);
        let (t76139, t76144, t76153, t76158) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1502::<F>(t231, t2782, t2783, t76136, t4500, t62967, t23168, t39598, t686, t72, t10530, t23172);
    (t76125, t76127, t76131, t76134, t76139, t76144, t76153, t76158)
}
