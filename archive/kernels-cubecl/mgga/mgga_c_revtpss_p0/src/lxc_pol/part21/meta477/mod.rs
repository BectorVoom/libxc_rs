//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2043;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2044;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta477<F: Float>(t15118: F, t918: F, t2889: F, t4614: F, t1606: F, t2439: F, t4580: F, t689: F, t4575: F, t2852: F, t4186: F, t606: F, t2850: F, t128: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15119, t15121, t15123, t15125) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2043::<F>(t15118, t918, t2889, t4614, t1606, t2439, t4580, t689);
        let t15127 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2044::<F>(t4575, t689);
        let (t15128, t15129, t15130, t15131, t15132) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2045::<F>(t15127, t2852, t4186, t606, t2850, t128);
    (t15119, t15121, t15123, t15125, t15127, t15128, t15129, t15130, t15131, t15132)
}
