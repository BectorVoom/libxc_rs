//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta363 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1268;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1269;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta363(t15154: f64, t2908: f64, t141: f64, t15158: f64, t930: f64, t4625: f64, t698: f64, t4622: f64, t15130: f64, t15137: f64, t15142: f64, t15147: f64, t15151: f64, t15156: f64, t15160: f64, t15135: f64, t11341: f64, t15140: f64, t15145: f64, t15149: f64, t1593: f64, t2435: f64, t4584: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15163, t15166, t15168, t15170, t15173, t15175) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1268(t15154, t2908, t141, t15158, t930, t4625, t698, t4622, t15130, t15137, t15142, t15147, t15151, t15156, t15160);
        let (t15178, t15181, t15184, t15187, t15189) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1269(t15135, t2908, t141, t11341, t15140, t15145, t930, t15149, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1270(t4584, t689);
    (t15163, t15166, t15168, t15170, t15173, t15175, t15178, t15181, t15184, t15187, t15189, t15191)
}
