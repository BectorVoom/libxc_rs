//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta421 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1572;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1573;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1574;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1575;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1576;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1578;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1579;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta421<F: Float>(t12306: F, t689: F, t13099: F, t159: F, t128: F, t43767: F, t2435: F, t3364: F, t3362: F, t39449: F, t3360: F, t3367: F, t1120: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43813: F, t12309: F, t12305: F, t43777: F, t1123: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t43858 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1572::<F>(t12306, t689);
        let t43862 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1573::<F>(t13099, t159, t128, t43767);
        let t43865 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1574::<F>(t2435, t3364);
        let (t43869, t43871) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1575::<F>(t3362, t39449, t128, t3360);
        let (t43875, t43877) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1576::<F>(t3367, t39449, t1120, t128);
        let (t43880, t43881) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577::<F>(t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877, t43813);
        let t43883 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1578::<F>(t12309, t689);
        let t43886 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1579::<F>(t12305, t128, t43777);
        let t43888 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1580::<F>(t1123, t9292);
    (t43858, t43862, t43865, t43869, t43871, t43875, t43877, t43880, t43881, t43883, t43886, t43888)
}
