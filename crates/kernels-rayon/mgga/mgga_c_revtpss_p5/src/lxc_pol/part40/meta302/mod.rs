//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1066;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta302(t10981: f64, t10982: f64, t2455: f64, t9285: f64, t2454: f64, t2829: f64, t779: f64, t689: f64, t2444: f64, t887: f64, t252: f64, t2769: f64, t786: f64, t2771: f64, t676: f64, t123: f64, t2435: f64, t2448: f64, t2440: f64, t2439: f64, t866: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10984, t10987, t10989, t10992, t10994) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1066(t10981, t10982, t2455, t9285, t2454, t2829, t779, t689, t2444, t887, t252, t2769);
        let (t10995, t10998, t11000, t11004, t11008) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1067(t10994, t786, t2771, t676, t123, t2435, t2448, t2440, t887, t2439, t866, t225);
    (t10984, t10987, t10989, t10992, t10995, t10998, t11000, t11004, t11008)
}
