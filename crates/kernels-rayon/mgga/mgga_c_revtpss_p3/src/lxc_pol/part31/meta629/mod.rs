//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta629(t25526: f64, t4820: f64, t15769: f64, t25522: f64, t15687: f64, t25515: f64, t3317: f64, t25525: f64, t4878: f64, t27450: f64, t3173: f64, t16035: f64, t25580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t100048, t100051, t100054, t100055, t100074, t100078, t100092) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083(t25526, t4820, t15769, t25522, t15687, t25515, t3317, t25525, t4878, t27450, t3173, t16035, t25580);
    (t100048, t100051, t100054, t100055, t100074, t100078, t100092)
}
