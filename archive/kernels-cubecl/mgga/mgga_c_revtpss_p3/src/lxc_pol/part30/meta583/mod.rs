//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2037;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta583<F: Float>(t7284: F, t94600: F, t25884: F, t686: F, t72: F, t25895: F, t7243: F, t9292: F, t1032: F, t4066: F, t1955: F, t25878: F, t2453: F, t3908: F, t7275: F, t1399: F, t2434: F, t25880: F, t25899: F, t3924: F, t676: F, t2022: F, t9646: F, t9648: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94602, t94605, t94608, t94609, t94610, t94613) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2037::<F>(t7284, t94600, t25884, t686, t72, t25895, t7243, t9292, t1032, t4066, t1955, t25878);
        let (t94616, t94634, t94635, t94640, t94641, t94648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2038::<F>(t2453, t3908, t7275, t1399, t2434, t25880, t25899, t3924, t676, t2022, t9646, t9648);
    (t94602, t94605, t94608, t94609, t94610, t94613, t94616, t94634, t94635, t94640, t94641, t94648)
}
