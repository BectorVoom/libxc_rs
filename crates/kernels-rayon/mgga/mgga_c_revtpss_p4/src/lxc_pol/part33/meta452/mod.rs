//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1643;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1644;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1645;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta452(t20823: f64, t5268: f64, t1042: f64, t5265: f64, t5274: f64, t1774: f64, t3362: f64, t4181: f64, t12787: f64, t12916: f64, t6689: f64, t3718: f64, t17661: f64, t5401: f64, t1214: f64, t1715: f64, t1250: f64, t17353: f64, t5052: f64, t5406: f64, t1794: f64, t3617: f64, t372: f64, t5047: f64, t3603: f64, t5284: f64, t5332: f64, t3720: f64, t12866: f64, t17340: f64, t17342: f64, t17693: f64, t17729: f64, t3711: f64, t5340: f64, t11249: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20914, t20917, t20923, t20926, t20927) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1643(t20823, t5268, t1042, t5265, t5274, t1774, t3362, t4181, t12787, t12916, t6689, t3718);
        let (t20929, t20934, t20938, t20941, t20945, t20946) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1644(t17661, t5401, t1214, t1715, t1250, t17353, t5052, t5406, t1794, t3617, t372, t5047);
        let (t20947, t20952, t20955) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1645(t20945, t20946, t3603, t5284, t5332, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1646(t11249, t6628);
    (t20914, t20923, t20926, t20929, t20934, t20938, t20941, t20947, t20952, t20955, t20956)
}
