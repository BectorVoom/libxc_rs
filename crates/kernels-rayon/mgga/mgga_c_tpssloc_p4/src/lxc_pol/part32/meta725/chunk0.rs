//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2329/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2329(t1210: f64, t24721: f64, t29593: f64, t27700: f64, t95422: f64, t2132: f64, t2136: f64, t5398: f64, t19040: f64, t7345: f64, t18392: f64, t27617: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104355 = t24721 * t1210 * t29593;
    let t104364 = t95422 * t27700;
    let t104367 = t2132 * t5398 * t2136;
    let t104369 = t7345 * t19040;
    let t104371 = t7345 * t18392;
    let t104375 = t27617 * t4993;
    (t104355, t104364, t104367, t104369, t104371, t104375)
}
