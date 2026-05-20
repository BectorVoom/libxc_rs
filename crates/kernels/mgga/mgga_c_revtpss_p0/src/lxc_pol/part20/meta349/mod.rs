//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta349<F: Float>(t372: F, t5268: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F, t3603: F, t1204: F, t5477: F, t1269: F, t3781: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17799, t17845, t17846, t17847, t17852, t17853, t17854, t17864, t17879) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1277::<F>(t372, t5268, t13147, t487, t460, t12050, t13045, t13141, t3603, t1204, t5477, t1269, t3781);
    (t17799, t17845, t17846, t17847, t17852, t17853, t17854, t17864, t17879)
}
