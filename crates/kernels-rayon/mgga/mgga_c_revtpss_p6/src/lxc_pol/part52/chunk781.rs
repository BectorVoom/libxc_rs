//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 781/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk781(t1518: f64, t7553: f64, t117: f64, t7983: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t8118: f64, t38: f64) -> (f64, f64, f64, f64) {
    let t8124 = t7553 * t1518;
    let t8127 = t117 * t7983;
    let t8130 = 3.0_f64 * t1916 * t2115 + 3.0_f64 * t1918 * t2113 + 6.0_f64 * t572 * t8124 + 3.0_f64 * t572 * t8127 + t573 * t8118;
    let t8435 = t38 * t38;
    (t8124, t8127, t8130, t8435)
}
