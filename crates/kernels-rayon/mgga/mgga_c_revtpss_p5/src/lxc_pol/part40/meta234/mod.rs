//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta234(t1250: f64, t482: f64, t5284: f64, t1042: f64, t1038: f64, t1802: f64, t1244: f64, t1241: f64, t1121: f64, t1263: f64) -> (f64, f64, f64, f64, f64) {
        let (t5286, t5287, t5292, t5293, t5296) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk908(t1250, t482, t5284, t1042, t1038, t1802, t1244, t1241, t1121, t1263);
    (t5286, t5287, t5292, t5293, t5296)
}
