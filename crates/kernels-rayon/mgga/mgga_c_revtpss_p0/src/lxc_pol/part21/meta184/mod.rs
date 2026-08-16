//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1129;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta184(t45: f64, t1522: f64, t2398: f64, t1568: f64, t212: f64, t780: f64, t689: f64, t1569: f64, t786: f64, t789: f64, t1469: f64, t80: f64, t4186: f64, t606: f64, t766: f64, zeta_threshold: f64, t57: f64, t83: f64, t770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4334) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1129(t45, t1522, t2398, t1568, t212, t780, t689, t1569, t786, t789, t1469, t80, t4186, t606, t766, zeta_threshold);
        let (t4335, t4343) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1130(t57, t1469, t83, t4186, t606, t770, t4334, zeta_threshold);
    (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4343)
}
