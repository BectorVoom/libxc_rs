//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 868/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk868(t31332: f64, t865: f64, t23270: f64, t1888: f64, t2053: f64, t857: f64, t776: f64, t22986: f64, t6547: f64, t8538: f64, t2047: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31333 = t31332 * t865;
    let t31334 = t23270 * t31333;
    let t31335 = t1888 * t31334;
    let t31337 = t857 * t2053;
    let t31338 = t31337 * t776;
    let t31339 = t23270 * t31338;
    let t31340 = t22986 * t31339;
    let t31349 = t6547 * t8538;
    let t31366 = t214 * t2047;
    (t31333, t31334, t31335, t31337, t31338, t31339, t31340, t31349, t31366)
}
