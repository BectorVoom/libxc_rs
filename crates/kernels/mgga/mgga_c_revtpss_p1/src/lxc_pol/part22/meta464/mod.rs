//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta464<F: Float>(t11108: F, t1699: F, t3022: F, t4725: F, t11465: F, t1633: F, t3015: F, t981: F, t3026: F, t4719: F, t1695: F, t3075: F) -> (F, F, F, F, F, F) {
        let (t15566, t15571, t15573, t15575, t15577, t15578) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2145::<F>(t11108, t1699, t3022, t4725, t11465, t1633, t3015, t981, t3026, t4719, t1695, t3075);
    (t15566, t15571, t15573, t15575, t15577, t15578)
}
