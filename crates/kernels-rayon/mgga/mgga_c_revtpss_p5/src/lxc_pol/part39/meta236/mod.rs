//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta236 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk908;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk909;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta236(t482: f64, t5245: f64, t371: f64, t372: f64, t1234: f64, t1803: f64, t225: f64, t5219: f64, t480: f64, t3623: f64, t4890: f64, t3782: f64, t1794: f64, t3153: f64, t1248: f64, t471: f64, t3720: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t1791: f64, t3637: f64, t3667: f64, t3711: f64, t5293: f64, t5299: f64, t5304: f64, t5309: f64, t5313: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5318, t5320, t5323, t5326, t5327, t5330) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk908(t482, t5245, t371, t372, t1234, t1803, t225, t5219, t480, t3623, t4890);
        let (t5331, t5332) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk909(t3782, t5330, t1794, t3153);
        let (t5333, t5334, t5335, t5338) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk910(t1248, t471, t5332, t3720, t1222, t1235, t1238, t1252, t1261, t1791, t3637, t3667, t3711, t5293, t5299, t5304, t5309, t5313, t5320, t5323, t5327, t5331);
    (t5318, t5320, t5323, t5326, t5327, t5330, t5331, t5332, t5333, t5334, t5335, t5338)
}
