//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 676/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk676(t10222: f64, t2349: f64, t1934: f64, t2639: f64, t231: f64, t2739: f64, t10207: f64, t10209: f64, t10212: f64, t10215: f64, t1526: f64, t2320: f64, t2649: f64, t2666: f64, t2745: f64, t342: f64, t343: f64, t3806: f64) -> f64 {
    let t10223 = t10222 * t2349;
    let t10227 = t2639 * t1934;
    let t10231 = t231 * t2739;
    let t10235 = t2649 + t2745 + t10207 - t10209 / 18.0_f64 - t10212 / 6.0_f64 - t1526 * t3806 * t10215 / 9.0_f64 - t1526 * t2320 * t2666 / 6.0_f64 + t1526 * t2320 * t10223 / 6.0_f64 - t1526 * t2320 * t10227 / 12.0_f64 - t342 * t343 * t10231 / 4.0_f64;
    t10235
}
