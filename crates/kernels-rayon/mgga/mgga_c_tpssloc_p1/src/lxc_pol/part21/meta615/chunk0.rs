//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2390/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2390(t11998: f64, t28: f64, t517: f64, t32253: f64, t59: f64, t154: f64, t541: f64, t12364: f64, t3777: f64, t1354: f64, t12365: f64, t3853: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39877 = 1.0_f64 / t517 / t11998 / t28;
    let t39933 = t59 * t32253;
    let t39934 = t39933 * t154;
    let t39936 = 455.0_f64 / 243.0_f64 * t39934 * t541;
    let t39947 = t3777 * t12364;
    let t39948 = t39947 * t1354;
    let t39950 = t12365 * t3853;
    (t39877, t39933, t39934, t39936, t39947, t39948, t39950)
}
