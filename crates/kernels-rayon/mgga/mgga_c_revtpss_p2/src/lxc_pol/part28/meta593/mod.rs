//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta593 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta593(t46361: f64, t545: f64, t25880: f64, t9685: f64, t25895: f64, t25900: f64, t94596: f64, t25904: f64, t1032: f64, t9656: f64, t25875: f64, t25925: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94656, t94661, t94662, t94664, t94665, t94668, t94669, t94671) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2064(t46361, t545, t25880, t9685, t25895, t25900, t94596, t25904, t1032, t9656, t25875, t25925, t686, t72);
    (t94656, t94661, t94662, t94664, t94665, t94668, t94669, t94671)
}
