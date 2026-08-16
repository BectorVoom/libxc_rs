//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1709/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1709(t16253: f64, t16319: f64, t16361: f64, t16411: f64, t553: f64, t3901: f64, t5287: f64, t1352: f64, t16036: f64, t3856: f64, t5348: f64, t1834: f64, t3787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16413 = t16253 + t16319 + t16361 + t16411;
    let t16414 = t553 * t16413;
    let t16416 = t3901 * t5287;
    let t16419 = t16036 * t1352;
    let t16423 = t5348 * t3856;
    let t16428 = t3787 * t1834;
    (t16413, t16414, t16416, t16419, t16423, t16428)
}
