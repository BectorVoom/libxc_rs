//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1207/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1207(t23146: f64, t5593: f64, t1894: f64, t236: f64, t5544: f64, t6591: f64, t23056: f64, t5568: f64, t5527: f64, t23078: f64, t1484: f64, t1509: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28380 = t23146 * t5593;
    let t28383 = t1894 * t236 * t5544;
    let t28384 = t6591 * t28383;
    let t28386 = t23056 * t5568;
    let t28389 = t1894 * t236 * t5527;
    let t28390 = t23078 * t28389;
    let t28395 = t1484 * t1509 * t232;
    (t28380, t28383, t28384, t28386, t28389, t28390, t28395)
}
