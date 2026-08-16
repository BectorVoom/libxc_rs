//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta118 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk653;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk654;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta118(t1568: f64, t212: f64, t780: f64, t689: f64, t1569: f64, t786: f64, t789: f64, t1469: f64, t80: f64, t83: f64, t1544: f64, t221: f64, t2675: f64, t2674: f64, t1558: f64, t243: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk653(t1568, t212, t780, t689, t1569, t786, t789, t1469, t80, t83, t1544, t221, t2675);
        let (t4350, t4352, t4353) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk654(t2674, t4349, t1558, t243, t231);
    (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349, t4350, t4352, t4353)
}
