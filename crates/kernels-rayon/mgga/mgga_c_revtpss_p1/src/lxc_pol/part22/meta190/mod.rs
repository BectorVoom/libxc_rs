//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta190 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1211;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1212;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1213;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1214;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1215;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta190(t1568: f64, t233: f64, t869: f64, t689: f64, t72: f64, t686: f64, t874: f64, t822: f64, t1559: f64, t213: f64, t234: f64, t2776: f64, t2780: f64, t2787: f64, t2791: f64, t2796: f64, t2802: f64, t2806: f64, t2810: f64, t2815: f64, t4366: f64, t4424: f64, t4469: f64, t4494: f64, t4497: f64, t4501: f64, t4504: f64, t4514: f64, t820: f64, t837: f64, t879: f64, t868: f64, t1580: f64, t2437: f64, t2443: f64, t2446: f64, t2449: f64, t2460: f64, t2462: f64, t2468: f64, t2473: f64, t257: f64, t2765: f64, t4323: f64, t4326: f64, t4470: f64, t4474: f64, t4478: f64, t4482: f64, t4487: f64, t865: f64, t887: f64, t198: f64, t205: f64, t1544: f64, t262: f64, t1583: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4518, t4519, t4520, t4522, t4524, t4526) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1211(t1568, t233, t869, t689, t72, t686, t874, t822);
        let t4533 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1212(t1559, t213, t234, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2815, t4366, t4424, t4469, t4494, t4497, t4501, t4504, t4514, t4520, t4524, t4526, t820, t837, t879);
        let t4534 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1213(t4533, t868);
        let t4537 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1214(t1580, t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2765, t4323, t4326, t4470, t4474, t4478, t4482, t4487, t4534, t865, t887);
        let t4541 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1215(t198, t205);
        let (t4542, t4546) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1216(t1544, t262, t1583, t892);
    (t4518, t4519, t4520, t4522, t4524, t4526, t4533, t4534, t4537, t4541, t4542, t4546)
}
