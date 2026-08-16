//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 904/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk904(t480: f64, t8216: f64, t159: f64, t9437: f64, t157: f64, t1642: f64, t1984: f64, t378: f64, t7368: f64, t137: f64, t8906: f64, t542: f64, t7334: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39150 = t8216 * t480;
    let t39652 = 1.0_f64 / t9437 / t159;
    let t39653 = t157 * t39652;
    let t39693 = t1642 * t1984;
    let t39749 = t378 * t7368;
    let t39801 = 1.0_f64 / t8906 / t137;
    let t39852 = t542 * t7334;
    (t39150, t39652, t39653, t39693, t39749, t39801, t39852)
}
