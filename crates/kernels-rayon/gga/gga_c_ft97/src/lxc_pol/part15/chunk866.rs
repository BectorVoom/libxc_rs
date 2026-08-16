//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 866/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk866(t12168: f64, t70: f64, t170: f64, t180: f64, t178: f64, t2280: f64, t159: f64, t9437: f64, t157: f64, t10: f64, t11175: f64, t144: f64) -> (f64, f64, f64, f64, f64) {
    let t39600 = t12168 * t70;
    let t39603 = 220.0_f64 / 81.0_f64 * t170 * t39600 * t180;
    let t39616 = 1.0_f64 / t2280 / t178;
    let t39652 = 1.0_f64 / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39673 = t10 * t11175 * t144;
    (t39600, t39603, t39616, t39653, t39673)
}
