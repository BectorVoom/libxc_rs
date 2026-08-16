//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 750/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk750(t3602: f64, t3604: f64, t1042: f64, t1244: f64, t3598: f64, t3594: f64, t3153: f64, t471: f64, t1121: f64, t414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3605 = t3602 * t3604;
    let t3606 = t1042 * t3605;
    let t3609 = t1244 * t3598;
    let t3610 = t3594 * t3609;
    let t3611 = t3153 * t471;
    let t3612 = t3602 * t3611;
    let t3613 = t1042 * t3612;
    let t3617 = 1.0_f64 / t414 / t1121;
    (t3605, t3606, t3609, t3610, t3611, t3612, t3613, t3617)
}
