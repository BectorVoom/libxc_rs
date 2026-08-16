//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 793/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk793(t2393: f64, t763: f64, t2374: f64, t702: f64, t9454: f64, t2411: f64) -> (f64, f64, f64) {
    let t9467 = t2393 * t763;
    let t9469 = 0.21687162600603479684e-1_f64 * t2374 * t9467;
    let t9474 = t9454 * t702;
    let t9476 = 6.0_f64 * t2411 * t9474;
    (t9467, t9469, t9476)
}
