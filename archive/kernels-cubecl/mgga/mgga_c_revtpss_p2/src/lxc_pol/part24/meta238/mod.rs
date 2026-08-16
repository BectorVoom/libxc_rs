//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta238<F: Float>(t14103: F, t2457: F, t9674: F, t10073: F, t5737: F, t1882: F, t4114: F, t2482: F, t10069: F, t136: F, t1892: F, t3964: F) -> (F, F, F, F, F, F, F) {
        let (t14104, t14105, t14120, t14141, t14149, t14159, t14161) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk999::<F>(t14103, t2457, t9674, t10073, t5737, t1882, t4114, t2482, t10069, t136, t1892, t3964);
    (t14104, t14105, t14120, t14141, t14149, t14159, t14161)
}
