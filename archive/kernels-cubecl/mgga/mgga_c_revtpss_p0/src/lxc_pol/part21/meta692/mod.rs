//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta692<F: Float>(t43752: F, t439: F, t1160: F, t12408: F, t3519: F, t3522: F, t3444: F, t3451: F, t1156: F, t12428: F, t43813: F, t12547: F, t3523: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t45177, t45181, t45187, t45188, t45190, t45194, t45197, t45232, t45289) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2513::<F>(t43752, t439, t1160, t12408, t3519, t3522, t3444, t3451, t1156, t12428, t43813, t12547, t3523);
    (t45177, t45181, t45187, t45188, t45190, t45194, t45197, t45232, t45289)
}
