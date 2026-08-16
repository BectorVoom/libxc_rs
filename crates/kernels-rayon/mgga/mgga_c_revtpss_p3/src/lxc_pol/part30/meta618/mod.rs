//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta618(t7234: f64, t8995: f64, t28199: f64, t28021: f64, t7235: f64, t13648: f64, t2014: f64, t7312: f64, t25861: f64, t7732: f64, t2322: f64, t28056: f64, t25194: f64, t7898: f64, t25851: f64, t10416: f64, t7735: f64, t13435: f64, t27137: f64, t25856: f64, t4248: f64, t2034: f64, t49564: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98590, t98594, t98597, t98599, t98601) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2126(t7234, t8995, t28199, t28021, t7235, t13648, t2014, t7312, t25861, t7732, t2322, t28056);
        let (t98603, t98605, t98607, t98609, t98611, t98615, t98617) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2127(t25194, t7898, t25851, t7732, t10416, t7735, t13435, t2322, t27137, t25856, t4248, t2014, t2034, t49564);
    (t98590, t98594, t98597, t98599, t98601, t98603, t98605, t98607, t98609, t98611, t98615, t98617)
}
