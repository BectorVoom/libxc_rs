//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 737/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk737(t3783: f64, t394: f64, t1457: f64, t475: f64, t13328: f64, t484: f64, t380: f64, t470: f64, t140: f64, t446: f64, t480: f64, t12951: f64, t451: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14292 = t1457 * t1457;
    let t14293 = 1.0_f64 / t14292;
    let t14294 = t475 * t14293;
    let t14364 = t484 * t13328;
    let t14365 = t14364 * sigma0;
    let t14374 = 1.0_f64 / t470 / t380;
    let t14409 = 0.11791604938271604938e-1_f64 * t140 * t446 * t480;
    let t14484 = t451 * t12951;
    (t14265, t14294, t14365, t14374, t14409, t14484)
}
