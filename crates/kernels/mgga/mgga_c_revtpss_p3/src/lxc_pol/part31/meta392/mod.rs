//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta392<F: Float>(t1729: F, t2439: F, t5098: F, t698: F, t16708: F, t16710: F, t16712: F, t5095: F, t3523: F, t5180: F, t1737: F, t3451: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1433::<F>(t1729, t2439, t5098, t698, t16708, t16710, t16712, t5095, t3523, t5180, t1737, t3451);
    (t16876, t16892, t16893, t16915, t16916, t16917, t16929, t16931, t16988, t17010, t17011, t17023)
}
