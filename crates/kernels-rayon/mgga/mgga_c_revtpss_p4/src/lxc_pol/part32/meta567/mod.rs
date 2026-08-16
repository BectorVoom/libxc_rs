//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta567(t25823: f64, t4287: f64, t116: f64, t28683: f64, t101453: f64, t26179: f64, t28133: f64, t7706: f64, t95293: f64, t60224: f64, t7342: f64, t13272: f64, t26178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t101705, t101755, t101756, t101782, t101783, t101785, t101788) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1891(t25823, t4287, t116, t28683, t101453, t26179, t28133, t7706, t95293, t60224, t7342, t13272, t26178);
    (t101705, t101755, t101756, t101782, t101783, t101785, t101788)
}
