//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta159 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk968;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk969;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk970;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk971;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta159<F: Float>(t1568: F, t233: F, t869: F, t689: F, t72: F, t686: F, t874: F, t822: F, t1559: F, t213: F, t234: F, t2776: F, t2780: F, t2787: F, t2791: F, t2796: F, t2802: F, t2806: F, t2810: F, t2815: F, t4366: F, t4424: F, t4469: F, t4494: F, t4497: F, t4501: F, t4504: F, t4514: F, t820: F, t837: F, t879: F, t868: F, t1580: F, t2437: F, t2443: F, t2446: F, t2449: F, t2460: F, t2462: F, t2468: F, t2473: F, t257: F, t2765: F, t4323: F, t4326: F, t4470: F, t4474: F, t4478: F, t4482: F, t4487: F, t865: F, t887: F, t198: F, t205: F, t1544: F, t262: F, t1583: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4518, t4519, t4520, t4522, t4524, t4526) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk967::<F>(t1568, t233, t869, t689, t72, t686, t874, t822);
        let t4533 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk968::<F>(t1559, t213, t234, t2776, t2780, t2787, t2791, t2796, t2802, t2806, t2810, t2815, t4366, t4424, t4469, t4494, t4497, t4501, t4504, t4514, t4520, t4524, t4526, t820, t837, t879);
        let t4534 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk969::<F>(t4533, t868);
        let t4537 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk970::<F>(t1580, t213, t2437, t2443, t2446, t2449, t2460, t2462, t2468, t2473, t257, t2765, t4323, t4326, t4470, t4474, t4478, t4482, t4487, t4534, t865, t887);
        let t4541 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk971::<F>(t198, t205);
        let (t4542, t4546) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk972::<F>(t1544, t262, t1583, t892);
    (t4518, t4519, t4520, t4522, t4524, t4526, t4533, t4534, t4537, t4541, t4542, t4546)
}
