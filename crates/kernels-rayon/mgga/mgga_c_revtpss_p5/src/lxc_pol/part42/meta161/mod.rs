//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk704;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta161(t1534: f64, t177: f64, t762: f64, t162: f64, t2611: f64, t1469: f64, t189: f64, t606: f64, t2623: f64, t2621: f64, t2628: f64, t2632: f64, t4307: f64, t4310: f64, t4313: f64, t4316: f64, t4394: f64, t4396: f64, t4397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4398, t4399, t4400, t4401, t4402, t4403, t4405, t4406, t4407) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk704(t1534, t177, t762, t162, t2611, t1469, t189, t606, t2623, t2621, t2628, t2632, t4307, t4310, t4313, t4316, t4394, t4396, t4397);
    (t4398, t4399, t4400, t4401, t4402, t4403, t4405, t4406, t4407)
}
