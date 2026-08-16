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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta421(t12306: f64, t689: f64, t13099: f64, t159: f64, t128: f64, t43767: f64, t2435: f64, t3364: f64, t3362: f64, t39449: f64, t3360: f64, t3367: f64, t1120: f64, t43830: f64, t43832: f64, t43837: f64, t43841: f64, t43845: f64, t43849: f64, t43813: f64, t12309: f64, t12305: f64, t43777: f64, t1123: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43858 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1572(t12306, t689);
        let t43862 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1573(t13099, t159, t128, t43767);
        let t43865 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1574(t2435, t3364);
        let (t43869, t43871) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1575(t3362, t39449, t128, t3360);
        let (t43875, t43877) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1576(t3367, t39449, t1120, t128);
        let (t43880, t43881) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1577(t43830, t43832, t43837, t43841, t43845, t43849, t43858, t43862, t43865, t43871, t43877, t43813);
        let t43883 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1578(t12309, t689);
        let t43886 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1579(t12305, t128, t43777);
        let t43888 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1580(t1123, t9292);
    (t43858, t43862, t43865, t43869, t43871, t43875, t43877, t43880, t43881, t43883, t43886, t43888)
}
