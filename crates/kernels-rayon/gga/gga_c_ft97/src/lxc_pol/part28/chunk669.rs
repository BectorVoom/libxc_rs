//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 669/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk669(t23714: f64, t3392: f64, t1013: f64, t422: f64, t379: f64, t538: f64, t920: f64, t423: f64, t554: f64, t1008: f64, t72: f64, t5579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26692 = t3392 * t23714;
    let t26695 = t422 * t1013;
    let t26696 = t26695 * t379;
    let t26700 = t920 * t538;
    let t26701 = t423 * t26700;
    let t26705 = t920 * t554;
    let t26706 = t423 * t26705;
    let t26714 = t1008 * t554;
    let t26715 = t72 * t26714;
    let t26716 = t5579 * t26715;
    (t26692, t26696, t26700, t26701, t26705, t26706, t26714, t26715, t26716)
}
