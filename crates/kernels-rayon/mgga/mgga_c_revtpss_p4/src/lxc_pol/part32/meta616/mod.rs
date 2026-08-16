//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta616(t22081: f64, t26028: f64, t22085: f64, t22048: f64, t27940: f64, t22089: f64, t22146: f64, t26004: f64, t6884: f64, t6850: f64, t94513: f64, t22041: f64, t7252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108526, t108528, t108531, t108533, t108535, t108537, t108539, t108541) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1956(t22081, t26028, t22085, t22048, t27940, t22089, t22146, t26004, t6884, t6850, t94513, t22041, t7252);
    (t108526, t108528, t108531, t108533, t108535, t108537, t108539, t108541)
}
