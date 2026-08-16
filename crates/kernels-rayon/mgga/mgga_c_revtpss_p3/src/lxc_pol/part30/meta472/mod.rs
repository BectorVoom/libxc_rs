//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1782;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1783;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta472(t2471: f64, t7018: f64, t25331: f64, t7058: f64, t25309: f64, t7063: f64, t7060: f64, t25296: f64, t7064: f64, t2435: f64, t7015: f64, t251: f64, t786: f64, t1032: f64, t2769: f64, t233: f64, t122: f64, t1949: f64, t72: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25362, t25364, t25365, t25366, t25368, t25371, t25372) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1780(t2471, t7018, t25331, t7058, t25309, t7063, t7060, t25296, t7064, t2435, t7015, t251, t786);
        let (t25373, t25374) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1781(t1032, t2769, t233);
        let t25375 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1782(t25372, t25374);
        let t25377 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1783(t122, t1949, t72);
        let t25378 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1784(t2466, t25377);
    (t25362, t25364, t25365, t25366, t25368, t25371, t25372, t25373, t25374, t25375, t25377, t25378)
}
