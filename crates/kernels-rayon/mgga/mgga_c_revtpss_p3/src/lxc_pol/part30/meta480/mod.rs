//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1807;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1808;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta480(t25851: f64, t651: f64, t1936: f64, t3813: f64, t4254: f64, t7003: f64, t1310: f64, t7002: f64, t2033: f64, t530: f64, t1450: f64, t3829: f64, t2014: f64, t555: f64, t7063: f64, t1032: f64, t4075: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25853, t25856, t25858, t25860, t25861, t25863, t25864, t25865) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1807(t25851, t651, t1936, t3813, t4254, t7003, t1310, t7002, t2033, t530, t1450, t3829);
        let (t25866, t25868, t25875) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1808(t25864, t25865, t2014, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1809(t1032, t4075, t545);
    (t25853, t25856, t25858, t25860, t25861, t25863, t25865, t25866, t25868, t25875, t25876, t25877)
}
