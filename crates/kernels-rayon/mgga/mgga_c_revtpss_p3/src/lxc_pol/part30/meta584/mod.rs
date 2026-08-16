//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2039;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta584(t25875: f64, t94394: f64, t94398: f64, t46361: f64, t545: f64, t25880: f64, t9685: f64, t25895: f64, t25900: f64, t94596: f64, t25904: f64, t1032: f64, t9656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94649, t94650, t94656, t94661, t94662, t94664, t94665, t94667) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2039(t25875, t94394, t94398, t46361, t545, t25880, t9685, t25895, t25900, t94596, t25904, t1032, t9656);
    (t94649, t94650, t94656, t94661, t94662, t94664, t94665, t94667)
}
