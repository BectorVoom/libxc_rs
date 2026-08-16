//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2615;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2616;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta660(t1250: f64, t5052: f64, t17353: f64, t17661: f64, t5406: f64, t1794: f64, t3617: f64, t372: f64, t5047: f64, t3603: f64, t5284: f64, t5332: f64, t3720: f64, t12866: f64, t17340: f64, t17342: f64, t17693: f64, t17729: f64, t20914: f64, t20917: f64, t20923: f64, t20927: f64, t20929: f64, t20934: f64, t3711: f64, t5340: f64, t11249: f64, t6628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20937, t20938, t20941, t20944, t20945, t20946, t20947, t20950, t20951) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2615(t1250, t5052, t17353, t17661, t5406, t1794, t3617, t372, t5047, t3603, t5284, t5332);
        let (t20952, t20955) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2616(t20951, t3720, t12866, t17340, t17342, t17693, t17729, t20914, t20917, t20923, t20927, t20929, t20934, t20938, t20941, t20947, t3711, t5340);
        let t20956 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2617(t11249, t6628);
    (t20937, t20938, t20941, t20944, t20945, t20946, t20947, t20950, t20951, t20952, t20955, t20956)
}
