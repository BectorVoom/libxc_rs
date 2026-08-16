//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta160 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk772;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk773;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk774;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta160(t4311: f64, t707: f64, t2498: f64, t2518: f64, t2522: f64, t2526: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t2610: f64, t4300: f64, t4301: f64, t4304: f64, t4307: f64, t4310: f64, t45: f64, t1522: f64, t2398: f64, t1568: f64, t212: f64, t780: f64, t689: f64, t1569: f64, t786: f64, t789: f64, t1469: f64, t80: f64, t4186: f64, t606: f64, t766: f64, zeta_threshold: f64, t57: f64, t83: f64, t770: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4313, t4314) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk772(t4311, t707, t2498, t2518, t2522, t2526, t2562, t2569, t2579, t2587, t2610, t4300, t4301, t4304, t4307, t4310);
        let (t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4334) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk773(t45, t1522, t2398, t1568, t212, t780, t689, t1569, t786, t789, t1469, t80, t4186, t606, t766, zeta_threshold);
        let (t4335, t4343) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk774(t57, t1469, t83, t4186, t606, t770, t4334, zeta_threshold);
    (t4313, t4314, t4316, t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4343)
}
