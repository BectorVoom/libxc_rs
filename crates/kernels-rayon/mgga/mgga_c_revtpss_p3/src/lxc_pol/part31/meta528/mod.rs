//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1903;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta528(t2007: f64, t4292: f64, t670: f64, t7883: f64, t1843: f64, t7002: f64, t651: f64, t2322: f64, t7742: f64, t4254: f64, t1310: f64, t7741: f64, t22496: f64, t8717: f64, t25082: f64, t1469: f64, t25129: f64, t25132: f64, t25137: f64, t4181: f64, t4186: f64, t6968: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28050, t28053, t28056, t28058, t28060, t28062, t28063) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1903(t2007, t4292, t670, t7883, t1843, t7002, t651, t2322, t7742, t4254, t1310, t7741);
        let (t28065, t28067, t28069, t28076) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1904(t28063, t651, t22496, t8717, t25082, t1469, t25129, t25132, t25137, t4181, t4186, t6968);
    (t28050, t28053, t28056, t28058, t28060, t28062, t28063, t28065, t28067, t28069, t28076)
}
