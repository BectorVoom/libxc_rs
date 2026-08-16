//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 910/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk910(t1240: f64, t799: f64, t66422: f64, t688: f64, t17818: f64, t17817: f64, t65693: f64, t1613: f64, t1689: f64, t2035: f64, t39: f64, t811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72397 = t799 * t1240;
    let t79528 = t66422 * t688;
    let t79529 = t79528 * t17818;
    let t79641 = t17817 * t65693;
    let t79931 = t1689 * t1613;
    let t82957 = t811 * t39 * t2035;
    (t72397, t79528, t79529, t79641, t79931, t82957)
}
