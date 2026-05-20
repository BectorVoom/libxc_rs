//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta386<F: Float>(t5098: F, t698: F, t16708: F, t16710: F, t16712: F, t5095: F, t12472: F, t1744: F, t3523: F, t5180: F, t12555: F, t1756: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16965, t16988, t16997) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1732::<F>(t5098, t698, t16708, t16710, t16712, t5095, t12472, t1744, t3523, t5180, t12555, t1756);
    (t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16965, t16988, t16997)
}
