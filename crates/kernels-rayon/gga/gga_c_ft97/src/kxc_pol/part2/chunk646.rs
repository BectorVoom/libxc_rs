//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 646/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk646(t170: f64, t180: f64, t8715: f64, t645: f64, t8640: f64, t2252: f64, t342: f64, t511: f64, t1526: f64, t1944: f64, t7705: f64, t1948: f64, t630: f64) -> (f64, f64, f64, f64, f64) {
    let t8718 = 20.0_f64 / 27.0_f64 * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8759 = t342 * t2252 * t511 / 18.0_f64;
    let t8761 = t1526 * t7705 * t1944;
    let t8764 = t342 * t630 * t1948;
    (t8718, t8719, t8759, t8761, t8764)
}
