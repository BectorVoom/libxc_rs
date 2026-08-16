//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 714/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk714(t3700: f64, t9483: f64, t701: f64, t173: f64, t2440: f64, t3691: f64, t420: f64, t9651: f64, t2248: f64, t703: f64, t3813: f64, t8715: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13595 = t9483 * t3700;
    let t13596 = t701 * t13595;
    let t13598 = t173 * t2440;
    let t13599 = t13598 * t3691;
    let t13600 = t701 * t13599;
    let t13601 = 0.56749874115226337448e-2_f64 * t13600;
    let t13605 = t420 * t9651;
    let t13609 = t2248 * t2440;
    let t13616 = t2248 * t703;
    let t13628 = t8715 * t3813;
    (t13596, t13600, t13601, t13605, t13609, t13616, t13628)
}
