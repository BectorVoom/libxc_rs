//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2039/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2039(t25875: f64, t94394: f64, t94398: f64, t46361: f64, t545: f64, t25880: f64, t9685: f64, t25895: f64, t25900: f64, t94596: f64, t25904: f64, t1032: f64, t9656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94649 = t25875 * t94394;
    let t94650 = t94649 * t94398;
    let t94656 = t46361 * t545;
    let t94661 = t25880 * t9685;
    let t94662 = t25895 * t94661;
    let t94664 = t94596 * t25900;
    let t94665 = t25904 * t94664;
    let t94667 = t1032 * t9656;
    (t94649, t94650, t94656, t94661, t94662, t94664, t94665, t94667)
}
