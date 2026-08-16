//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta595(t14991: f64, t95936: f64, t7407: f64, t99373: f64, t2435: f64, t28390: f64, t102993: f64, t25411: f64, t2470: f64, t28359: f64, t7064: f64, t7997: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t103396, t103399, t103400, t103404, t103421, t103422, t103424) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1927(t14991, t95936, t7407, t99373, t2435, t28390, t102993, t25411, t2470, t28359, t7064, t7997, t822);
    (t103396, t103399, t103400, t103404, t103421, t103422, t103424)
}
