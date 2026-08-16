//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1071;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1072;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta290<F: Float>(t378: F, t6235: F, t1678: F, t4746: F, t6343: F, t994: F, t19462: F, t6461: F, t698: F, t6464: F, t6467: F, t6422: F, t689: F, t6426: F, t6430: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20178, t20191, t20204, t20211, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1071::<F>(t378, t6235, t1678, t4746, t6343, t994, t19462, t6461, t698, t6464, t6467, t6422, t689);
        let t20285 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1072::<F>(t6426, t689);
        let t20287 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1073::<F>(t6430, t689);
    (t20178, t20191, t20204, t20211, t20276, t20278, t20280, t20283, t20285, t20287)
}
