//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2456;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2457;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta705<F: Float>(t47371: F, t786: F, t10115: F, t1441: F, t4093: F, t9292: F, t1432: F, t1433: F, t39497: F, t10111: F, t1428: F, t588: F, t10022: F, t2453: F, t268: F, t39644: F, t546: F, t555: F, t8779: F, t4107: F, t9288: F, t10107: F, t3964: F, t9285: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47372, t47381, t47389, t47395, t47417) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2456::<F>(t47371, t786, t10115, t1441, t4093, t9292, t1432, t1433, t39497, t10111, t1428, t588);
        let (t47429, t47442, t47444, t47450) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2457::<F>(t10022, t2453, t268, t39644, t546, t555, t8779, t1432, t4107, t9288, t10107, t3964, t9285);
    (t47372, t47381, t47389, t47395, t47417, t47429, t47442, t47444, t47450)
}
