//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta484<F: Float>(t1285: F, t70994: F, t1121: F, t6587: F, t17395: F, t17400: F, t20809: F, t372: F, t3655: F, t6598: F, t6602: F, t5436: F) -> (F, F, F, F, F, F, F) {
        let (t70995, t71029, t71081, t71112, t71187, t71192, t71275) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1476::<F>(t1285, t70994, t1121, t6587, t17395, t17400, t20809, t372, t3655, t6598, t6602, t5436);
    (t70995, t71029, t71081, t71112, t71187, t71192, t71275)
}
