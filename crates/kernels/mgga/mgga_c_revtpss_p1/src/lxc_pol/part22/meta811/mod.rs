//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta811 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta811<F: Float>(t39515: F, t4083: F, t10043: F, t9303: F, t10139: F, t281: F, t4056: F, t543: F, t68: F, t14192: F, t555: F, t10115: F, t1441: F, t4093: F, t9292: F, t10065: F, t10073: F, t1432: F, t1433: F, t39497: F, t10061: F, t10069: F, t10111: F, t1428: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47351, t47352, t47364, t47371, t47381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2914::<F>(t39515, t4083, t10043, t9303, t10139, t281, t4056, t543, t68, t14192, t555, t10115, t1441);
        let (t47389, t47391, t47395, t47403, t47413, t47417) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2915::<F>(t4093, t9292, t10065, t10073, t1432, t1433, t39497, t10061, t10069, t10111, t1428, t588);
    (t47351, t47352, t47364, t47371, t47381, t47389, t47391, t47395, t47403, t47413, t47417)
}
