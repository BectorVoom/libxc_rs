//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 591/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk591(t2116: f64, t3281: f64, t502: f64, t507: f64, t512: f64, t1050: f64, t120: f64, t1599: f64, t526: f64) -> (f64, f64, f64, f64) {
    let t3282 = t3281 * t2116;
    let t3285 = t512 * t502 * t507;
    let t3288 = t120 * t1599 * t1050;
    let t3290 = t120 * t526;
    (t3282, t3285, t3288, t3290)
}
