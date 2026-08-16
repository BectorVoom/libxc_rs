//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1946/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1946(t24601: f64, t27437: f64, t24590: f64, t8002: f64, t3247: f64, t497: f64, t3961: f64, t24574: f64, t8067: f64, t1184: f64, t1715: f64, t24745: f64, t7363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27438 = t24601 * t27437;
    let t27441 = t24590 * t8002;
    let t27444 = t497 * t3247;
    let t27445 = t27444 * t3961;
    let t27446 = t24601 * t27445;
    let t27451 = t24574 * t8067;
    let t27453 = t1715 * t1184;
    let t27454 = t24745 * t7363;
    (t27438, t27441, t27444, t27445, t27446, t27451, t27453, t27454)
}
