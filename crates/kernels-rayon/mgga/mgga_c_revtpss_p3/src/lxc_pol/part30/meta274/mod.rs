//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1212;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1213;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1214;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1215;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta274(t670: f64, t7226: f64, t7228: f64, t7230: f64, t7584: f64, t7586: f64, t118: f64, t1310: f64, t1453: f64, t2127: f64, t2163: f64, t2165: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t6990: f64, t6992: f64, t6995: f64, t7005: f64, t7236: f64, t7241: f64, t7314: f64, t7317: f64, t7591: f64, t7683: f64, t3: f64, param_d: f64, t1461: f64, t2170: f64, t573: f64, t7329: f64, t7333: f64, t7336: f64, t38: f64, t4173: f64, t1497: f64, t84: f64, t77: f64, t1470: f64, t603: f64, t1493: f64, t76: f64, t1937: f64, t4248: f64, t1518: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7687, t7690) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1212(t670, t7226, t7228, t7230, t7584, t7586, t118, t1310, t1453, t2127, t2163, t2165, t508, t569, t649, t651, t671, t6990, t6992, t6995, t7005, t7236, t7241, t7314, t7317, t7591, t7683);
        let (t7691, t7696) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1213(t3, t7690, param_d);
        let (t7700, t7702, t7705, t7706) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1214(t1461, t2170, t573, t7329, t7333, t7336, t7696, t38, t4173, t1497, t84, t77);
        let (t7709, t7719, t7731, t7732) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1215(t1470, t603, t1493, t76, t1937, t4248, t1518, t94);
    (t7687, t7690, t7691, t7696, t7700, t7702, t7705, t7706, t7709, t7719, t7731, t7732)
}
