//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1053/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1053(t11453: f64, t3955: f64, t2731: f64, t3978: f64, t967: f64, t3973: f64, t10416: f64, t3972: f64, t3931: f64, t10412: f64, t2761: f64, t8444: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11454 = t11453 * t3955;
    let t11456 = t2731 * t11454 / 2304.0_f64;
    let t11457 = t11453 * t3978;
    let t11459 = t967 * t11457 / 1728.0_f64;
    let t11460 = t11453 * t3973;
    let t11462 = 5.0_f64 / 10368.0_f64 * t967 * t11460;
    let t11463 = t3972 * t10416;
    let t11464 = t3931 * t11463;
    let t11467 = t3972 * t10412;
    let t11468 = t3931 * t11467;
    let t11475 = t2761 * t8444;
    (t11456, t11459, t11462, t11464, t11468, t11475)
}
