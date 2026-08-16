//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk859;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk860;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta221(t2339: f64, t5891: f64, t1504: f64, t2349: f64, t100: f64, t5823: f64, t1479: f64, t1509: f64, t2357: f64, tau1: f64, t108: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t97: f64, t114: f64, t655: f64, t2335: f64, t4261: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5892, t5895, t5896, t5899, t5902, t5907, t5908, t5911) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk859(t2339, t5891, t1504, t2349, t100, t5823, t1479, t1509, t2357, tau1);
        let t5915 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk860(t108, t5911, t105, t109, t1507, t1510, t5896, t5899, t5902, t5908, t97);
        let (t5916, t5920) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk861(t114, t5915, t655, t2335, t4261, t5892, t69);
    (t5892, t5895, t5896, t5899, t5902, t5907, t5911, t5915, t5916, t5920)
}
