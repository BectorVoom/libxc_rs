//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1055;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta292(t159: f64, t3181: f64, t2851: f64, t631: f64, t45: f64, t1071: f64, t3057: f64, t992: f64, t338: f64, t378: f64, t3056: f64, t988: f64, t1031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11142, t11144, t11150, t11187, t11200, t11201, t11223) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1055(t159, t3181, t2851, t631, t45, t1071, t3057, t992, t338, t378, t3056, t988);
        let (t11224, t11239) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1056(t11223, t378, t1031);
    (t11142, t11144, t11150, t11187, t11200, t11201, t11223, t11224, t11239)
}
