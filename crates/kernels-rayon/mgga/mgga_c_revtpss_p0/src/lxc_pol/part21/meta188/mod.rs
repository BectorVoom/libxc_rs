//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta188 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1142;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1143;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1144;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1145;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta188(t4402: f64, t606: f64, t4401: f64, t2623: f64, t2621: f64, t2628: f64, t2632: f64, t4307: f64, t4310: f64, t4313: f64, t4316: f64, t4394: f64, t4396: f64, t4397: f64, t4400: f64, t225: f64, t4376: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t775: f64, t4343: f64, t832: f64, t1553: f64, t1555: f64, t229: f64, t830: f64, t833: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4403, t4405, t4406, t4407) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1142(t4402, t606, t4401, t2623, t2621, t2628, t2632, t4307, t4310, t4313, t4316, t4394, t4396, t4397, t4400);
        let (t4409, t4415) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1143(t225, t4376, t4407, t227, t73);
        let t4416 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1144(t1544, t853);
        let (t4417, t4420, t4423) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1145(t4416, t775, t4343, t832, t1553, t1555, t227, t229, t4409, t4415, t830, t833);
        let t4424 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1146(t231, t4423);
    (t4403, t4405, t4406, t4409, t4415, t4416, t4417, t4420, t4423, t4424)
}
