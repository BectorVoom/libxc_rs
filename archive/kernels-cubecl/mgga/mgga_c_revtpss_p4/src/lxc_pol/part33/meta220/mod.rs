//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1009;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta220<F: Float>(t5: F, t1497: F, t2247: F, t4173: F, t5812: F, t5816: F, t5872: F, t603: F, t91: F, t117: F, t1518: F, t94: F, t1843: F, t1513: F) -> (F, F, F, F, F, F) {
        let (t5876, t5877) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1008::<F>(t5, t1497, t2247, t4173, t5812, t5816, t5872, t603, t91, t117);
        let t5883 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1009::<F>(t1518);
        let (t5884, t5887, t5891) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1010::<F>(t5883, t94, t1518, t1843, t1513);
    (t5876, t5877, t5883, t5884, t5887, t5891)
}
