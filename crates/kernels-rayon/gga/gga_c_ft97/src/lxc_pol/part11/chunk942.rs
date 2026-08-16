//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 942/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk942(t643: f64, t8376: f64, t12168: f64, t70: f64, t170: f64, t180: f64, t2253: f64, t8711: f64, t3628: f64, t645: f64, t2294: f64, t8621: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39575 = t8376 * t643;
    let t39600 = t12168 * t70;
    let t39603 = 220.0_f64 / 81.0_f64 * t170 * t39600 * t180;
    let t39604 = t2253 * t8711;
    let t39606 = t3628 * t645;
    let t39608 = t2294 * t2294;
    let t39613 = t2253 * t8621;
    (t39575, t39600, t39603, t39604, t39606, t39608, t39613)
}
