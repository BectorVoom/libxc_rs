//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2170/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2170(t7759: f64, t822: f64, t25310: f64, t27279: f64, t27186: f64, t93321: f64, t93374: f64, t122: f64, t72: f64, t2466: f64, t25387: f64, t231: f64, t4533: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99334 = t822 * t7759;
    let t99342 = 0.14456046980341999104e-1_f64 * t25310 * t27279;
    let t99344 = 0.14456046980341999104e-1_f64 * t93321 * t27186;
    let t99346 = 0.25702851531048074406e-1_f64 * t93374 * t27186;
    let t99348 = t7759 * t72 * t122;
    let t99349 = t99348 * t2466;
    let t99351 = 0.51405703062096148812e-1_f64 * t25387 * t99349;
    let t99360 = t4533 * t836 * t231;
    (t99334, t99342, t99344, t99346, t99348, t99349, t99351, t99360)
}
