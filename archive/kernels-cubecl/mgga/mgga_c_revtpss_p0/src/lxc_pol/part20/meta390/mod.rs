//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1426;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1427;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1428;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1429;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1430;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1431;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1432;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1433;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1434;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1435;
use chunk10::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1436;
use chunk11::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta390<F: Float>(t41306: F, t2435: F, t2863: F, t2854: F, t11170: F, t689: F, t11146: F, t41308: F, t41312: F, t41316: F, t41320: F, t41323: F, t41327: F, t11852: F, t159: F, t128: F, t41297: F, t41301: F, t904: F, t2850: F, t41277: F, t11142: F, t41271: F, t41248: F, t41253: F, t41258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41329, t41330) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1426::<F>(t41306, t2435, t2863);
        let t41332 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1427::<F>(t2435, t2854);
        let t41334 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1428::<F>(t11170, t689);
        let t41336 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1429::<F>(t11146, t689);
        let (t41338, t41339) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1430::<F>(t41308, t41312, t41316, t41320, t41323, t41327, t41329, t41330, t41332, t41334, t41336, t11852, t159);
        let t41341 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1431::<F>(t128, t41297, t41339);
        let t41344 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1432::<F>(t128, t41301, t904);
        let t41347 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1433::<F>(t128, t2850, t41277);
        let t41350 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1434::<F>(t11142, t128, t41271);
        let t41353 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1435::<F>(t11142, t128, t41248);
        let t41356 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1436::<F>(t128, t41253, t904);
        let t41359 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1437::<F>(t128, t2850, t41258);
    (t41330, t41332, t41334, t41336, t41338, t41341, t41344, t41347, t41350, t41353, t41356, t41359)
}
