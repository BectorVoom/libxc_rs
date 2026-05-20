//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1783;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1784;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1785;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta489<F: Float>(t122: F, t2022: F, t72: F, t3916: F, t25878: F, t2435: F, t7243: F, t555: F, t786: F, t25877: F, t1385: F, t2028: F, t25875: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25880, t25881, t25882, t25893, t25894) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1783::<F>(t122, t2022, t72, t3916, t25878, t2435, t7243, t555, t786);
        let t25895 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1784::<F>(t25877, t25894);
        let (t25896, t25898) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1785::<F>(t25881, t25895, t1385, t2028);
        let t25899 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1786::<F>(t25875, t25898);
    (t25880, t25881, t25882, t25893, t25894, t25895, t25896, t25898, t25899)
}
