//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1266/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1266(t46361: f64, t545: f64, t25880: f64, t9685: f64, t25895: f64, t25900: f64, t94596: f64, t25904: f64, t1032: f64, t9656: f64, t25875: f64, t25925: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    let t94662 = t25895 * t94661;
    let t94664 = t94596 * t25900;
    let t94665 = t25904 * t94664;
    let t94667 = t1032 * t9656;
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94671 = t25925 * t72 * t686;
    (t94656, t94661, t94662, t94664, t94665, t94668, t94669, t94671)
}
