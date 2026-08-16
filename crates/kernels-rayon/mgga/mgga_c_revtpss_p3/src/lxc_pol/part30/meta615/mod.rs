//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2120;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta615(t1937: f64, t98487: f64, t27123: f64, t6993: f64, t25803: f64, t7898: f64, t2033: f64, t47672: f64, t1907: f64, t4144: f64, t28196: f64, t27833: f64, t7313: f64, t3829: f64, t28167: f64, t8717: f64, t25082: f64, t28197: f64, t73488: f64, t13625: f64, t33651: f64, t25090: f64, t28187: f64, t7235: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98489, t98491, t98494, t98499, t98501) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2120(t1937, t98487, t27123, t6993, t25803, t7898, t2033, t47672, t1907, t4144, t28196, t27833, t7313);
        let (t98522, t98525, t98528, t98530, t98532) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2121(t1907, t3829, t28167, t8717, t25082, t28197, t73488, t13625, t33651, t25090, t7898, t28187, t7235);
    (t98489, t98491, t98494, t98499, t98501, t98522, t98525, t98528, t98530, t98532)
}
