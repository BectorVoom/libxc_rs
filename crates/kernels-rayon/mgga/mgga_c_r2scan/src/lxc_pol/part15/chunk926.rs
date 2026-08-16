//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 926/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk926(t597: f64, t874: f64, t10682: f64, t10680: f64, t2262: f64, t2333: f64, t3263: f64, t3275: f64, t2065: f64, t3446: f64, t3453: f64, t2068: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10683 = t597 * t874;
    let t10684 = t10682 * t10683;
    let t10685 = t10680 * t10684;
    let t10687 = t2333 * t2262;
    let t10689 = t3275 * t3263 * t10687;
    let t10690 = t10689 / 4.0_f64;
    let t10692 = t3446 * t3453 * t2065;
    let t10695 = t3446 * t3453 * t2068;
    (t10683, t10684, t10685, t10687, t10690, t10692, t10695)
}
