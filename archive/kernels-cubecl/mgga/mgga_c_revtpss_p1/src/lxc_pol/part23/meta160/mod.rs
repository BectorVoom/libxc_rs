//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta160 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk973;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk974;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk975;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk976;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk977;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk978;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk979;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk980;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk981;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta160<F: Float>(t1544: F, t2404: F, t1583: F, t2411: F, t1940: F, t198: F, t207: F, t2403: F, t2621: F, t2628: F, t2632: F, t4316: F, t4343: F, t4394: F, t4396: F, t4397: F, t4400: F, t4405: F, t4406: F, t4537: F, t4541: F, t4542: F, t4546: F, t765: F, t775: F, t890: F, t892: F, t4314: F, t2: F, t265: F, t580: F, t1593: F, t689: F, t1469: F, t2852: F, t606: F, t2850: F, t128: F, t2857: F, t904: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4553, t4556) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk973::<F>(t1544, t2404, t1583, t2411);
        let t4559 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk974::<F>(t1940, t198, t207, t2403, t2621, t2628, t2632, t4316, t4343, t4394, t4396, t4397, t4400, t4405, t4406, t4537, t4541, t4542, t4546, t4553, t4556, t765, t775, t890, t892);
        let t4560 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk975::<F>(t4314, t4559);
        let (t4568, t4571) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk976::<F>(t2, t265, t580, t1593, t689);
        let t4573 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk977::<F>(t1469, t2852);
        let t4574 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk978::<F>(t4573, t606);
        let (t4575, t4576) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk979::<F>(t2850, t4574, t128);
        let t4578 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk980::<F>(t1469, t2857);
        let t4579 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk981::<F>(t4578, t606);
        let (t4580, t4581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk982::<F>(t4579, t904, t128);
    (t4556, t4560, t4568, t4571, t4573, t4574, t4575, t4576, t4578, t4579, t4580, t4581)
}
