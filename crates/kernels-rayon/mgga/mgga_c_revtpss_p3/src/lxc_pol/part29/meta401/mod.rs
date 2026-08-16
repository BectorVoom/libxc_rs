//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1443;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1444;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta401(t11150: f64, t1469: f64, t2251: f64, t2850: f64, t128: f64, t4573: f64, t904: f64, t2908: f64, t141: f64, t930: f64, t4625: f64, t698: f64, t4622: f64, t15130: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15154, t15156) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1443(t11150, t1469, t2251, t2850, t128);
        let (t15158, t15160) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1444(t2251, t4573, t904, t128);
        let (t15163, t15166, t15168, t15170, t15173, t15175) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1445(t15154, t2908, t141, t15158, t930, t4625, t698, t4622, t15130, t15137, t15142, t15147, t15151, t15156, t15160);
    (t15154, t15156, t15158, t15160, t15163, t15166, t15168, t15170, t15173, t15175)
}
