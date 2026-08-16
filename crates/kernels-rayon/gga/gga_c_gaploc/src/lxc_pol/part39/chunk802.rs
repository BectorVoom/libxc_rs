//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 802/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk802(t12223: f64, t2562: f64, t883: f64, t943: f64, t2558: f64, t3732: f64, t13870: f64, t169: f64, t299: f64, t706: f64, t270: f64, t13883: f64, t738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13934 = t2562 * t883 * t12223;
    let t13935 = t943 * t13934;
    let t13937 = t3732 * t2558;
    let t13938 = t943 * t13937;
    let t13941 = t13870 * t169 * t299;
    let t13942 = t706 * t13941;
    let t13944 = 0.76905262301422242837e-2_f64 * t270 * t13942;
    let t13945 = t738 * t13883;
    (t13934, t13935, t13937, t13938, t13941, t13942, t13944, t13945)
}
