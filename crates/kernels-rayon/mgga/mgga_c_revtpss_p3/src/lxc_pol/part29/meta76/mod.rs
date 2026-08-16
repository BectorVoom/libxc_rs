//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta76 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk479;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk480;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk481;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta76(t1469: f64, t60: f64, t1474: f64, t1480: f64, t44: f64, t56: f64, t61: f64, t626: f64, t38: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t71: f64, t85: f64, t5: f64, t1466: f64, t603: f64, t91: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1486, t1487, t1490, t1491, t1493) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk479(t1469, t60, t1474, t1480, t44, t56, t61, t626, t38, t633, t637);
        let (t1494, t1497) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk480(t1493, t77, t1471, t1487, t71, t85);
        let t1501 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk481(t5, t1466, t1497, t603, t91);
        let t1502 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk482(t117, t1501);
    (t1486, t1487, t1490, t1491, t1493, t1494, t1497, t1501, t1502)
}
