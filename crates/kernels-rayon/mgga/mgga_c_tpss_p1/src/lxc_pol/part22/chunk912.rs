//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 912/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk912(t2162: f64, t2364: f64, t219: f64, t2399: f64, t810: f64, t73: f64, t2398: f64, t768: f64, t242: f64, t2675: f64, t2704: f64, t946: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8330 = t2162 * t2364;
    let t8339 = t2399 * t219;
    let t8346 = t810 * t810;
    let t8347 = 1.0_f64 / t8346;
    let t8348 = t73 * t8347;
    let t8372 = t768 * t2398;
    let t8430 = t242 * t2675 * t2704;
    let t8431 = t946 * t8430;
    (t8330, t8339, t8346, t8347, t8348, t8372, t8431)
}
