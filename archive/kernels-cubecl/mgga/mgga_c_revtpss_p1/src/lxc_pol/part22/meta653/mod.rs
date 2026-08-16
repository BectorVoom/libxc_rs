//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta653<F: Float>(t20800: F, t5341: F, t3720: F, t5333: F, t1263: F, t6587: F, t1122: F, t1042: F, t3172: F, t6624: F, t1247: F, t1032: F, t6564: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20801, t20802, t20805, t20806, t20809, t20810, t20811, t20816, t20817, t20819) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2603::<F>(t20800, t5341, t3720, t5333, t1263, t6587, t1122, t1042, t3172, t6624, t1247, t1032, t6564);
    (t20801, t20802, t20805, t20806, t20809, t20810, t20811, t20816, t20817, t20819)
}
