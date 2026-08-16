//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2447/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2447(t3105: f64, t3223: f64, t11960: f64, t351: f64, t361: f64, t369: f64, t1041: f64, t11262: f64, t3135: f64, t1033: f64, t1036: f64, t1038: f64) -> (f64, f64, f64, f64) {
    let t42571 = t3223 * t3105;
    let t42576 = t351 * t361 * t11960 * t369;
    let t42580 = t1041 * t11262 * t3135;
    let t42584 = t1033 * t1036 * t11960 * t1038;
    (t42571, t42576, t42580, t42584)
}
