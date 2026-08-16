//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 801/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk801(t12345: f64, t12595: f64, t515: f64, t1053: f64, t2157: f64, t2179: f64, t3565: f64, t609: f64, t2180: f64, t9439: f64, t3478: f64, t379: f64) -> (f64, f64, f64, f64, f64) {
    let t12596 = t12345 + t12595;
    let t12597 = t515 * t12596;
    let t12599 = t1053 * t2157;
    let t12600 = t2179 * t12599;
    let t12602 = t3565 * t609;
    let t12603 = t2179 * t12602;
    let t12605 = t1053 * t2180;
    let t12606 = t9439 * t12605;
    let t12609 = t3478 * t379;
    (t12597, t12600, t12603, t12606, t12609)
}
