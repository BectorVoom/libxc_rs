//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk884;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk885;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta200(t1248: f64, t471: f64, t5332: f64, t3720: f64, t1222: f64, t1235: f64, t1238: f64, t1252: f64, t1261: f64, t1791: f64, t3637: f64, t3667: f64, t3711: f64, t5293: f64, t5299: f64, t5304: f64, t5309: f64, t5313: f64, t5320: f64, t5323: f64, t5327: f64, t5331: f64, t3767: f64, t5330: f64, t3603: f64, t1774: f64, t1250: f64, t1794: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5333, t5334, t5335, t5338) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk884(t1248, t471, t5332, t3720, t1222, t1235, t1238, t1252, t1261, t1791, t3637, t3667, t3711, t5293, t5299, t5304, t5309, t5313, t5320, t5323, t5327, t5331);
        let t5340 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk885(t3767, t5330);
        let (t5341, t5342, t5343, t5346, t5347, t5348, t5351) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk886(t1248, t3603, t5332, t3720, t1774, t1250, t1794, t73);
    (t5333, t5334, t5335, t5338, t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351)
}
