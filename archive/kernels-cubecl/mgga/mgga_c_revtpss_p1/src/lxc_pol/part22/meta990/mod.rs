//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta990 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3368;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3369;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3370;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3371;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3372;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3373;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3374;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3375;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta990<F: Float>(t18947: F, t689: F, t60754: F, t905: F, t128: F, t904: F, t2435: F, t6093: F, t2852: F, t60717: F, t2850: F, t6097: F, t63330: F, t6101: F, t41330: F, t41332: F, t52047: F, t52049: F, t52051: F, t63399: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t63447 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3368::<F>(t18947, t689);
        let (t63449, t63451) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3369::<F>(t60754, t905, t128, t904);
        let t63453 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3370::<F>(t2435, t6093);
        let (t63455, t63457) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3371::<F>(t2852, t60717, t128, t2850);
        let t63459 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3372::<F>(t2435, t6097);
        let t63462 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3373::<F>(t128, t63330, t904);
        let t63464 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3374::<F>(t2435, t6101);
        let t63466 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3375::<F>(t41330, t41332, t52047, t52049, t52051, t63399, t63447, t63451, t63453, t63457, t63459, t63462, t63464);
    (t63447, t63449, t63451, t63453, t63455, t63457, t63459, t63462, t63464, t63466)
}
