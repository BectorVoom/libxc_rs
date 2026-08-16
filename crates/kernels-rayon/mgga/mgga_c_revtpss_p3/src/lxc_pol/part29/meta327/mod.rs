//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta327(t10994: f64, t786: f64, t2771: f64, t676: f64, t123: f64, t2435: f64, t2448: f64, t2440: f64, t887: f64, t2439: f64, t866: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10995, t10997, t10998, t11000, t11004, t11006, t11007, t11008) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1238(t10994, t786, t2771, t676, t123, t2435, t2448, t2440, t887, t2439, t866, t225);
    (t10995, t10997, t10998, t11000, t11004, t11006, t11007, t11008)
}
