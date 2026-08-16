//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1865;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1866;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta514(t1096: f64, t7821: f64, t7160: f64, t1976: f64, t4772: f64, t7145: f64, t1982: f64, t4930: f64, t1000: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1986: f64, t25634: f64, t25658: f64, t25692: f64, t25695: f64, t27557: f64, t27568: f64, t4743: f64, t4764: f64, t4773: f64, t4941: f64, t5016: f64, t7102: f64, t7137: f64, t7140: f64, t7151: f64, t7817: f64, t988: f64, t1035: f64, t7810: f64, t1043: f64, t1089: f64, t27418: f64, t342: f64, t1678: f64, t3140: f64, t1078: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27576, t27579, t27580, t27587, t27592) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1865(t1096, t7821, t7160, t1976, t4772, t7145, t1982, t4930, t1000, t1647, t1652, t1696, t1978, t1986, t25634, t25658, t25692, t25695, t27557, t27568, t4743, t4764, t4773, t4941, t5016, t7102, t7137, t7140, t7151);
        let (t27595, t27599, t27604, t27606, t27609) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1866(t1096, t7817, t7160, t7821, t988, t7145, t1035, t7810, t1043, t1089, t1982, t27418);
        let (t27616, t27621) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1867(t342, t7810, t1678, t3140, t1078, t1982);
    (t27576, t27579, t27580, t27587, t27592, t27595, t27599, t27604, t27606, t27609, t27616, t27621)
}
