//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 506/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk506(t2294: f64, t259: f64, t363: f64, t364: f64, t358: f64, t265: f64, t2098: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2295 = t259 * t2294;
    let t2298 = t363 * t363;
    let t2300 = 1.0_f64 / t364 / t2298;
    let t2301 = t358 * t2300;
    let t2302 = t2301 * t265;
    let t2304 = 1.0_f64 / t9 / t2098;
    (t2295, t2298, t2300, t2301, t2302, t2304)
}
