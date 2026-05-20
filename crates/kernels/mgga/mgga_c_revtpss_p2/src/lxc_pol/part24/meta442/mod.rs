//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1399;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta442<F: Float>(t220: F, t47273: F, t2482: F, t27: F, t9991: F, t1389: F, t3964: F, t40604: F, t39515: F, t4083: F, t14192: F, t555: F, t786: F, t1432: F, t1433: F, t39497: F, t10111: F, t1428: F, t588: F, t10022: F, t2453: F, t268: F, t39644: F, t546: F, t8779: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47274, t47293, t47337, t47351, t47371) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1399::<F>(t220, t47273, t2482, t27, t9991, t1389, t3964, t40604, t39515, t4083, t14192, t555);
        let (t47372, t47395, t47417, t47429, t47442) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1400::<F>(t47371, t786, t1432, t1433, t39497, t10111, t1428, t588, t10022, t2453, t268, t39644, t546, t555, t8779);
    (t47274, t47293, t47337, t47351, t47372, t47395, t47417, t47429, t47442)
}
