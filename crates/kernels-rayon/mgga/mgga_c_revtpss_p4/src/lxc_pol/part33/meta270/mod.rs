//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta270 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1210;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1211;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1212;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1213;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta270(t3: f64, t7690: f64, t1461: f64, t2170: f64, t573: f64, t7329: f64, t7333: f64, t7336: f64, t38: f64, t4173: f64, t1497: f64, t84: f64, param_d: f64, t77: f64, t1470: f64, t603: f64, t1493: f64, t76: f64, t1937: f64, t4248: f64, t1518: f64, t94: f64, t1843: f64, t1936: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7691, t7696, t7700, t7702, t7705) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209(t3, t7690, t1461, t2170, t573, t7329, t7333, t7336, t38, t4173, t1497, t84, param_d);
        let t7706 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1210(t77, t7705);
        let t7709 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1211(t1470, t603);
        let t7719 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1212(t1493, t76);
        let (t7731, t7732) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1213(t1937, t4248, t1518, t94);
        let (t7734, t7735) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1214(t1937, t7732, t1843, t1936);
    (t7691, t7696, t7700, t7702, t7705, t7706, t7709, t7719, t7731, t7732, t7734, t7735)
}
