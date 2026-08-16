//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1810;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1811;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1812;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1813;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1814;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta481<F: Float>(t25875: F, t25877: F, t122: F, t2022: F, t72: F, t3916: F, t4131: F, t7296: F, t1444: F, t7274: F, t2435: F, t7243: F, t555: F, t786: F, t1385: F, t2028: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t25878 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1810::<F>(t25875, t25877);
        let t25880 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1811::<F>(t122, t2022, t72);
        let t25881 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1812::<F>(t25880, t3916);
        let (t25882, t25884, t25885, t25889, t25893, t25894) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1813::<F>(t25878, t25881, t2022, t4131, t7296, t1444, t7274, t2435, t7243, t555, t786);
        let t25895 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1814::<F>(t25877, t25894);
        let (t25896, t25898) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1815::<F>(t25881, t25895, t1385, t2028);
    (t25878, t25880, t25881, t25882, t25884, t25885, t25889, t25893, t25894, t25895, t25896, t25898)
}
