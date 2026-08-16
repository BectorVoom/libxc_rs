//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2013;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta595<F: Float>(t2018: F, t40688: F, t46808: F, t7256: F, t9784: F, t1445: F, t2439: F, t25916: F, t25877: F, t94390: F, t94385: F, t9675: F, t7289: F, t94377: F, t7285: F, t9288: F, t7284: F, t7243: F, t9292: F, t2453: F, t3908: F, t7275: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94569, t94571, t94580, t94589, t94590) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2013::<F>(t2018, t40688, t46808, t7256, t9784, t1445, t2439, t25916, t25877, t94390, t94385, t9675);
        let (t94591, t94593, t94600, t94602, t94608, t94616) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2014::<F>(t94589, t94590, t7289, t94377, t7285, t9288, t7284, t7243, t9292, t2453, t3908, t7275);
    (t94569, t94571, t94580, t94589, t94590, t94591, t94593, t94600, t94602, t94608, t94616)
}
