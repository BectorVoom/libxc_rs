//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2071/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2071(t7058: f64, t99321: f64, t7759: f64, t822: f64, t25310: f64, t27279: f64, t27186: f64, t93321: f64, t93374: f64, t122: f64, t72: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99323 = 0.14456046980341999104e-1_f64 * t7058 * t99321;
    let t99334 = t822 * t7759;
    let t99342 = 0.14456046980341999104e-1_f64 * t25310 * t27279;
    let t99344 = 0.14456046980341999104e-1_f64 * t93321 * t27186;
    let t99346 = 0.25702851531048074406e-1_f64 * t93374 * t27186;
    let t99348 = t7759 * t72 * t122;
    let t99349 = t99348 * t2466;
    (t99323, t99334, t99342, t99344, t99346, t99348, t99349)
}
