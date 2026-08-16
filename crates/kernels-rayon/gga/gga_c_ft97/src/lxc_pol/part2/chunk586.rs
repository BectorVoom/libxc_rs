//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 586/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk586(t1131: f64, t2: f64, t2372: f64, t713: f64, t192: f64, t3821: f64, t743: f64, t2481: f64, t2482: f64, t2484: f64, t3139: f64, t3908: f64, t3911: f64, t3914: f64, t3918: f64, t3922: f64, t3925: f64, t3927: f64, t462: f64, t92: f64) -> (f64, f64, f64, f64) {
    let t3930 = t2 * t1131;
    let t3932 = t2372 * t3930 * t713;
    let t3936 = t192 * t743 * t3821;
    let t3938 = t2481 + t2482 / 9.0_f64 + t2484 / 3.0_f64 + t3908 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t462 * t3911 + t462 * t3914 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t3918 + 2.0_f64 / 3.0_f64 * t3139 * t3922 + t3925 / 3.0_f64 + t462 * t3927 / 3.0_f64 + 2.0_f64 * t462 * t3932 - t92 * t3936;
    (t3930, t3932, t3936, t3938)
}
