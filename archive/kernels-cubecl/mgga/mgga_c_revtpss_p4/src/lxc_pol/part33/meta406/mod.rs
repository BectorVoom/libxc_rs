//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta406<F: Float>(t473: F, t5412: F, t13147: F, t487: F, t460: F, t12050: F, t13045: F, t13141: F, t3603: F, t1284: F, t5216: F, t1770: F, t3766: F) -> (F, F, F, F, F, F, F) {
        let (t17821, t17846, t17847, t17853, t17854, t17861, t17934) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1457::<F>(t473, t5412, t13147, t487, t460, t12050, t13045, t13141, t3603, t1284, t5216, t1770, t3766);
    (t17821, t17846, t17847, t17853, t17854, t17861, t17934)
}
