//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 984/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk984(t3431: f64, t725: f64, t681: f64, t2112: f64, t3642: f64, t774: f64, t8305: f64, t1364: f64, t782: f64, t2177: f64, t2174: f64, t1378: f64) -> (f64, f64, f64, f64, f64) {
    let t10564 = t725 * t3431;
    let t10566 = 8.0_f64 * t681 * t10564;
    let t10568 = 8.0_f64 * t2112 * t3642;
    let t10572 = t8305 * t774;
    let t10573 = t1364 * t782;
    let t10574 = t10573 * t2177;
    let t10575 = t10572 * t10574;
    let t10578 = t2174 * t774;
    let t10579 = t1378 * t782;
    (t10566, t10568, t10575, t10578, t10579)
}
