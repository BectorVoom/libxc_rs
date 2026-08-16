//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1241/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1241(t10469: f64, t10474: f64, t363: f64, t10401: f64, t23417: f64, t3186: f64, t10383: f64, t1926: f64, t10472: f64, t10478: f64, t23535: f64, t6753: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82989 = t10469 * t10474 * t363;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83028 = 5.0_f64 / 1296.0_f64 * t1926 * t10383;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    let t83065 = t10472 * t6753 * t10478;
    (t82989, t83015, t83016, t83028, t83054, t83058, t83065)
}
