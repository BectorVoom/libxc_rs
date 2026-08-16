//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta356(t10845: f64, t4430: f64, t1558: f64, t853: f64, t2749: f64, t2662: f64, t2661: f64, t4352: f64, t837: f64, t4416: f64, t221: f64, t2485: f64, t4424: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14716, t14718, t14719, t14722, t14723, t14726, t14727, t14730, t14732) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1298(t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837, t4416, t221, t2485, t4424);
    (t14716, t14718, t14719, t14722, t14723, t14726, t14727, t14730, t14732)
}
