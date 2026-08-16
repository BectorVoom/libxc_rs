//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1903/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1903(t1874: f64, t28002: f64, t4028: f64, t7461: f64, t19451: f64, t1774: f64, t7467: f64, t652: f64, t2006: f64, t6361: f64, t1807: f64, t7722: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28038 = 4.0_f64 * t28002 * t1874;
    let t28040 = 4.0_f64 * t4028 * t7461;
    let t28042 = 2.0_f64 * t19451 * t1874;
    let t28045 = t1774 * t7467;
    let t28047 = 4.0_f64 * t652 * t28045;
    let t28051 = t6361 * t2006;
    let t28053 = t1807 * t7722;
    (t28038, t28040, t28042, t28045, t28047, t28051, t28053)
}
