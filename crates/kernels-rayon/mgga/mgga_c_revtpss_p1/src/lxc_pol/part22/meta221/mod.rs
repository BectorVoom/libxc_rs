//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta221 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1400;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1401;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1402;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1403;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1404;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1405;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1406;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta221(t1235: f64, t5362: f64, t1219: f64, t1778: f64, t1225: f64, t4186: f64, t1012: f64, t1222: f64, t3657: f64, t3658: f64, t3679: f64, t3684: f64, t3718: f64, t5340: f64, t5343: f64, t5348: f64, t5354: f64, t5358: f64, t1010: f64, t1480: f64, t1715: f64, t3634: f64, t247: f64, t1261: f64, t1260: f64, t1785: f64, t3670: f64, t3719: f64, t5230: f64, t1802: f64, t369: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5363, t5366, t5368, t5369, t5372) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1400(t1235, t5362, t1219, t1778, t1225, t4186, t1012, t1222, t3657, t3658, t3679, t3684, t3718, t5340, t5343, t5348, t5354, t5358);
        let t5373 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1401(t1010, t1480);
        let t5378 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1402(t1715, t3634, t247);
        let (t5379, t5381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1403(t1261, t5378, t1260, t1785);
        let t5384 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1404(t1260, t3670);
        let t5386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1405(t3719, t5230, t247);
        let t5390 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1406(t1802, t369, t475);
    (t5363, t5366, t5368, t5369, t5372, t5373, t5378, t5379, t5381, t5384, t5386, t5390)
}
