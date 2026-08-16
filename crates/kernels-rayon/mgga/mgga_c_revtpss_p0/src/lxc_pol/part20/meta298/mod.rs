//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1172;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1173;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1174;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1175;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1176;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1177;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta298(t268: f64, t404: f64, t7021: f64, t1123: f64, t2435: f64, t3364: f64, t689: f64, t3369: f64, t3373: f64, t159: f64, t3617: f64, t12257: f64, t128: f64, t12269: f64, t3360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t12295 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1172(t268, t404, t7021);
        let (t12296, t12297) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1173(t12295, t1123, t2435);
        let t12299 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1174(t3364, t689);
        let t12301 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1175(t3369, t689);
        let t12303 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1176(t3373, t689);
        let (t12305, t12306, t12307) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1177(t159, t3617, t12257, t128);
        let (t12309, t12310) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1178(t12269, t3360, t128);
    (t12295, t12296, t12297, t12299, t12301, t12303, t12305, t12306, t12307, t12309, t12310)
}
