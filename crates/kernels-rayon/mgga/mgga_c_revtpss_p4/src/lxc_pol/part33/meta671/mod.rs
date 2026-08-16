//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta671 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2199;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta671(t18227: f64, t7742: f64, t28063: f64, t4248: f64, t28182: f64, t7898: f64, t29499: f64, t7235: f64, t2014: f64, t29498: f64, t32737: f64, t27137: f64, t7732: f64, t2322: f64, t29502: f64, t4254: f64, t5517: f64, t651: f64, t7741: f64, t101417: f64, t7900: f64, t196: f64, t197: f64, t22525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109043, t109045, t109047, t109049, t109052, t109054) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2199(t18227, t7742, t28063, t4248, t28182, t7898, t29499, t7235, t2014, t29498, t32737, t27137, t7732);
        let (t109058, t109060, t109063, t109074, t109077) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2200(t2322, t29502, t4254, t5517, t651, t7741, t101417, t2014, t7900, t196, t197, t22525);
    (t109043, t109045, t109047, t109049, t109052, t109054, t109058, t109060, t109063, t109074, t109077)
}
