//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 516/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk516(t2222: f64, t730: f64, t200: f64, t202: f64, t692: f64, t725: f64, t650: f64, t698: f64, t169: f64, t697: f64, t164: f64, t704: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2224 = 0.24415263074675393405e-3_f64 * t730 * t2222;
    let t2225 = 1.0_f64 / t200;
    let t2232 = 1.0_f64 / t202;
    let t2245 = t692 * t725;
    let t2250 = t650 * t698;
    let t2254 = t697 * t169;
    let t2255 = 1.0_f64 / t2254;
    let t2256 = t164 * t2255;
    let t2257 = t704 * t704;
    (t2224, t2225, t2232, t2245, t2250, t2255, t2256, t2257)
}
