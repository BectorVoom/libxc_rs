//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 654/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk654(t2281: f64, t71: f64, t118: f64, t7911: f64, t7944: f64, t3626: f64, t70: f64, t170: f64, t180: f64, t645: f64, t8640: f64, t2252: f64, t342: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8680 = t71 * t2281;
    let t8690 = 1.0_f64 / t118 / t7911;
    let t8698 = 0.44934037037037037036e0_f64 * t7944;
    let t8715 = t3626 * t70;
    let t8718 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8759 = t342 * t2252 * t511 / 18.0_f64;
    (t8680, t8690, t8698, t8715, t8718, t8719, t8759)
}
