//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta402<F: Float>(t1774: F, t3617: F, t372: F, t5268: F, t473: F, t5412: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F) -> (F, F, F, F, F, F, F) {
        let (t17794, t17799, t17821, t17845, t17846, t17847, t17852) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1771::<F>(t1774, t3617, t372, t5268, t473, t5412, t13147, t487, t460, t12050, t13045, t13141);
    (t17794, t17799, t17821, t17845, t17846, t17847, t17852)
}
