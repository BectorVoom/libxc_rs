//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2096/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2096(t23384: f64, t23582: f64, t23333: f64, t82431: f64, t23323: f64, t6683: f64, t23357: f64, t6680: f64, t23494: f64, t381: f64, t23403: f64, t23589: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83318 = t23384 * t23582;
    let t83329 = t82431 * t23333;
    let t83342 = t23323 * t6683;
    let t83344 = t6680 * t23357;
    let t83352 = t23494 * t381;
    let t83358 = t23384 * t23403;
    let t83364 = t23384 * t23589;
    (t83318, t83329, t83342, t83344, t83352, t83358, t83364)
}
