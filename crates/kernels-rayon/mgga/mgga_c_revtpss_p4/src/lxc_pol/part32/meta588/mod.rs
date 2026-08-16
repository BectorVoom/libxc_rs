//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta588(t2435: f64, t8011: f64, t25431: f64, t2439: f64, t93170: f64, t28347: f64, t686: f64, t72: f64, t25387: f64, t102980: f64, t93190: f64, t10073: f64, t26554: f64, t27198: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t102993, t102994, t103000, t103001, t103005, t103007, t103009, t103017) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1917(t2435, t8011, t25431, t2439, t93170, t28347, t686, t72, t25387, t102980, t93190, t10073, t26554, t27198);
    (t102993, t102994, t103000, t103001, t103005, t103007, t103009, t103017)
}
