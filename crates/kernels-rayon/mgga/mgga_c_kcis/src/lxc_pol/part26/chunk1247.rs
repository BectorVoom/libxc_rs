//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1247/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1247(t491: f64, t5747: f64, t16937: f64, t28484: f64, t27369: f64, t16941: f64, t28494: f64, t7908: f64, t28461: f64, t7904: f64, t1014: f64, t28528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98470 = t5747 * t491;
    let t98487 = t16937 * t28484;
    let t98489 = 0.20612155671296296296e-4_f64 * t27369 * t98487;
    let t98491 = t7908 * t16941 * t28494;
    let t98519 = 0.46336805555555555556e-3_f64 * t28461 * t7904;
    let t98522 = t1014 * t28528;
    (t98470, t98487, t98489, t98491, t98519, t98522)
}
