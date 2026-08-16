//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2477;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2478;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2479;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2480;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta675<F: Float>(t43813: F, t241: F, t281: F, t414: F, t39484: F, t403: F, t409: F, t12288: F, t698: F, t12316: F, t689: F, t12291: F, t12306: F, t13099: F, t159: F, t2435: F, t3364: F, t12309: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43814, t43816, t43817, t43821, t43828, t43830) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2477::<F>(t43813, t241, t281, t414, t39484, t403, t409, t12288, t698, t12316, t689);
        let t43832 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2478::<F>(t12291, t689);
        let t43858 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2479::<F>(t12306, t689);
        let (t43860, t43865) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2480::<F>(t13099, t159, t2435, t3364);
        let (t43881, t43883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2481::<F>(t43813, t12309, t689);
    (t43814, t43816, t43817, t43821, t43828, t43830, t43832, t43858, t43860, t43865, t43881, t43883)
}
