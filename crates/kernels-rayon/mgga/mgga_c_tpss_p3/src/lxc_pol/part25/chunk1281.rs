//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1281/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1281(t198: f64, t206: f64, t5848: f64, t5831: f64, t768: f64, t61024: f64, t61079: f64, t1811: f64, t31814: f64, t8096: f64, t61868: f64, t507: f64, t5935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t62610 = t198 * t206 * t5848;
    let t62671 = t768 * t5831;
    let t62690 = 595.0_f64 / 2592.0_f64 * t61024;
    let t62711 = 455.0_f64 / 648.0_f64 * t61079;
    let t62807 = t1811 * t31814;
    let t62829 = t5848 * t8096;
    let t63006 = 308.0_f64 / 27.0_f64 * t61868;
    let t63042 = t507 * t5935;
    (t62610, t62671, t62690, t62711, t62807, t62829, t63006, t63042)
}
