//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta659<F: Float>(t20921: F, t4181: F, t12787: F, t12916: F, t6689: F, t3718: F, t17661: F, t5401: F, t1214: F, t1715: F, t1250: F, t17353: F) -> (F, F, F, F, F, F, F, F) {
        let (t20922, t20923, t20926, t20927, t20929, t20932, t20933, t20934) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2614::<F>(t20921, t4181, t12787, t12916, t6689, t3718, t17661, t5401, t1214, t1715, t1250, t17353);
    (t20922, t20923, t20926, t20927, t20929, t20932, t20933, t20934)
}
