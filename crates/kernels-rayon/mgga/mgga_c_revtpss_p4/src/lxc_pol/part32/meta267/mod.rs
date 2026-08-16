//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1124;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1125;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1126;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1127;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta267(t1497: f64, t84: f64, t77: f64, t1470: f64, t603: f64, t1493: f64, t76: f64, t1518: f64, t94: f64, t1513: f64, t6998: f64, t1544: f64, t30: f64, t1549: f64, t7025: f64, t1561: f64, t7038: f64, t1565: f64, t7045: f64, t1568: f64, t1955: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7705, t7706) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1124(t1497, t84, t77);
        let t7709 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1125(t1470, t603);
        let t7719 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1126(t1493, t76);
        let t7732 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1127(t1518, t94);
        let (t7738, t7749, t7753, t7755, t7757, t7766) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1128(t1513, t6998, t1544, t30, t1549, t7025, t1561, t7038, t1565, t7045, t1568, t1955);
    (t7705, t7706, t7709, t7719, t7732, t7738, t7749, t7753, t7755, t7757, t7766)
}
