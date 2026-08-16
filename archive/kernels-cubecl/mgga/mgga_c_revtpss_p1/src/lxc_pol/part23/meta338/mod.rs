//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1639;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta338<F: Float>(t14224: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t1892: F, t4086: F, t786: F) -> (F, F, F, F, F, F, F, F) {
        let (t14225, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1639::<F>(t14224, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14231, t14233, t14238, t14239) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1640::<F>(t10022, t14230, t2782, t1892, t4086, t786);
    (t14225, t14227, t14229, t14230, t14231, t14233, t14238, t14239)
}
