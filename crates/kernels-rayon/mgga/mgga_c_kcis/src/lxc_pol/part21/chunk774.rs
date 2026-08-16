//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 774/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk774(t742: f64, t85: f64, t776: f64, t2429: f64, t2493: f64, t2484: f64, t2527: f64, t752: f64, t2718: f64, t873: f64, t872: f64, t206: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8931 = t85 * t742;
    let t8932 = t8931 * t776;
    let t8934 = t2429 * t2493;
    let t8936 = t2484 * t2527;
    let t8937 = t752 * t8936;
    let t8939 = t2718 * t873;
    let t8942 = t872 * t872;
    let t8943 = 1.0_f64 / t8942;
    let t8944 = t206 * t8943;
    (t8931, t8932, t8934, t8937, t8939, t8942, t8943, t8944)
}
