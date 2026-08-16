//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1105/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1105(t2247: f64, t34409: f64, t1518: f64, t2126: f64, t4147: f64, t8107: f64, t1497: f64, t8621: f64, t8881: f64, t1469: f64, t33268: f64, t8442: f64) -> (f64, f64, f64, f64, f64) {
    let t34410 = t2247 * t34409;
    let t34446 = t2126 * t1518;
    let t34495 = t4147 * t8107;
    let t34761 = t8621 * t8881 * t1497;
    let t34764 = t33268 * t1469;
    let t34765 = t8442 * t34764;
    (t34410, t34446, t34495, t34761, t34765)
}
