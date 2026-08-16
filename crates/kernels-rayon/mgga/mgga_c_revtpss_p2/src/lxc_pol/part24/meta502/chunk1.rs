//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1508/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1508(t10858: f64, t23257: f64, t221: f64, t23279: f64, t10703: f64, t2674: f64, t2661: f64, t2662: f64, t6035: f64, t61579: f64, t1559: f64, t18608: f64) -> (f64, f64, f64, f64) {
    let t76596 = t10858 * t23257;
    let t76613 = t221 * t23279;
    let t76615 = t2674 * t10703 * t76613;
    let t76619 = t2661 * t2662 * t61579 * t6035;
    let t76645 = t2661 * t2662 * t18608 * t1559;
    (t76596, t76615, t76619, t76645)
}
