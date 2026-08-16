//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1195/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1195(t2867: f64, t3275: f64, t37318: f64, t10680: f64, t10681: f64, t10683: f64, t2482: f64, t10673: f64, t10674: f64, t10676: f64, t11020: f64, t11545: f64) -> (f64, f64, f64, f64) {
    let t40338 = t3275 * t37318 * t2867 / 4.0_f64;
    let t40341 = t10680 * t10681 * t2482 * t10683;
    let t40342 = 0.72042316457491791906e-3_f64 * t40341;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40346 = 0.10248087766267884742e-3_f64 * t40345;
    let t40348 = 5.0_f64 / 16.0_f64 * t11020 * t11545;
    (t40338, t40342, t40346, t40348)
}
