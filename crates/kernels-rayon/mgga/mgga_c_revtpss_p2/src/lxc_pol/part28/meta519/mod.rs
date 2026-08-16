//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1940;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta519(t27634: f64, t7160: f64, t1078: f64, t11239: f64, t1035: f64, t1983: f64, t1668: f64, t1976: f64, t3153: f64, t4998: f64, t1043: f64, t1089: f64, t7828: f64, t1097: f64, t25591: f64, t25605: f64, t25640: f64, t25651: f64, t27595: f64, t27599: f64, t27606: f64, t27609: f64, t27616: f64, t27621: f64, t27627: f64, t27631: f64, t4758: f64, t7144: f64, t7156: f64, t7162: f64, t7167: f64, t7170: f64, t7174: f64, t7825: f64, t7833: f64, t7837: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27635, t27638, t27639, t27640, t27641, t27642) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1939(t27634, t7160, t1078, t11239, t1035, t1983, t1668, t1976, t3153);
        let (t27643, t27647, t27650) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1940(t27642, t4998, t1043, t1089, t7828, t1097, t25591, t25605, t25640, t25651, t27595, t27599, t27606, t27609, t27616, t27621, t27627, t27631, t27635, t27640, t4758, t7144, t7156, t7162, t7167, t7170, t7174, t7825, t7833, t7837);
        let t27651 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1941(t27641, t73);
    (t27635, t27638, t27639, t27640, t27641, t27642, t27643, t27647, t27650, t27651)
}
