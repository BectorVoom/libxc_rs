//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta191 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1217;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1218;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1219;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1220;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1221;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1222;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1223;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1224;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta191(t1544: f64, t2404: f64, t1583: f64, t2411: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t2621: f64, t2628: f64, t2632: f64, t4316: f64, t4343: f64, t4394: f64, t4396: f64, t4397: f64, t4400: f64, t4405: f64, t4406: f64, t4537: f64, t4541: f64, t4542: f64, t4546: f64, t765: f64, t775: f64, t890: f64, t892: f64, t4314: f64, t2: f64, t265: f64, t580: f64, t1593: f64, t689: f64, t1469: f64, t2852: f64, t606: f64, t2850: f64, t128: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4553, t4556) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1217(t1544, t2404, t1583, t2411);
        let t4559 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1218(t1940, t198, t207, t2403, t2621, t2628, t2632, t4316, t4343, t4394, t4396, t4397, t4400, t4405, t4406, t4537, t4541, t4542, t4546, t4553, t4556, t765, t775, t890, t892);
        let t4560 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1219(t4314, t4559);
        let (t4567, t4568, t4571) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1220(t2, t265, t580, t1593, t689);
        let t4573 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1221(t1469, t2852);
        let t4574 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1222(t4573, t606);
        let (t4575, t4576) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1223(t2850, t4574, t128);
        let t4578 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1224(t1469, t2857);
        let t4579 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1225(t4578, t606);
    (t4556, t4560, t4567, t4568, t4571, t4573, t4574, t4575, t4576, t4578, t4579)
}
