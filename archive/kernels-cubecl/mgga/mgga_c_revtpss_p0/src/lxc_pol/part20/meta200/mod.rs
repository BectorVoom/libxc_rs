//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk968;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta200<F: Float>(t4066: F, t72: F, t1432: F, t686: F, t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F) -> (F, F, F, F, F, F, F) {
        let (t10103, t10105, t10107, t10109, t10111) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk968::<F>(t4066, t72, t1432, t686, t136, t1419, t2457, t3964, t225, t9646);
        let (t10114, t10115) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk969::<F>(t10111, t1428, t22, t2452);
    (t10103, t10105, t10107, t10109, t10111, t10114, t10115)
}
