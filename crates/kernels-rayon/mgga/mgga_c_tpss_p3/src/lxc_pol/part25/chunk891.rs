//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 891/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk891(t8471: f64, t946: f64, t2464: f64, t265: f64, t2458: f64, t606: f64, t2719: f64, t72: f64, t2737: f64, t2798: f64, t2782: f64, t2762: f64, t774: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8472 = t946 * t8471;
    let t8491 = 1.0_f64 / t265 / t2464;
    let t8493 = 1.0_f64 / t2458 / t606;
    let t8507 = t2719 * t72;
    let t8508 = t2737 * t8507;
    let t8509 = t2798 * t8508;
    let t8514 = t2782 * t8508;
    let t8523 = t774 * t2762;
    (t8472, t8491, t8493, t8507, t8509, t8514, t8523)
}
